use super::{
    analytic_client::AnalyticClient,
    pattern_detector::{self, LearningResults, PatternDetector},
    types::{AnalyticServiceMessage, DetectionTask, LearningStatus, RequestType, ResponseType},
};
use super::types;

use crate::services::{
    metric_service::MetricService,
    segments_service::{self, Segment, SegmentType, SegmentsService, ID_LENGTH},
};
use crate::utils::{self, get_random_str};

use subbeat::metric::Metric;

use anyhow;

use tokio::sync::{mpsc, oneshot};

use futures::future;



// TODO: get this from pattern detector
const DETECTION_STEP: u64 = 10;

// TODO: now it's basically single analytic unit, service will operate on many AU
pub struct AnalyticService {
    metric_service: MetricService,
    segments_service: SegmentsService,
    learning_results: Option<LearningResults>,
    learning_status: LearningStatus,
    tx: mpsc::Sender<AnalyticServiceMessage>,
    rx: mpsc::Receiver<AnalyticServiceMessage>,

    // handlers
    learning_handler: Option<tokio::task::JoinHandle<()>>,

    // awaiters
    learning_waiters: Vec<DetectionTask>,
}

impl AnalyticService {
    pub fn new(
        metric_service: MetricService,
        segments_service: segments_service::SegmentsService,
    ) -> AnalyticService {
        let (tx, rx) = mpsc::channel::<AnalyticServiceMessage>(32);

        AnalyticService {
            metric_service,
            segments_service,
            // TODO: get it from persistance
            learning_results: None,
            learning_status: LearningStatus::Initialization,
            tx,
            rx,

            // handlers
            learning_handler: None,

            // awaiters
            learning_waiters: Vec::new(),
        }
    }

    pub fn get_client(&self) -> AnalyticClient {
        AnalyticClient::new(self.tx.clone())
    }

    fn run_detection_task(&self, task: DetectionTask) {
        // TODO: save handler of the task
        tokio::spawn({
            let lr = self.learning_results.as_ref().unwrap().clone();
            let ms = self.metric_service.clone();
            async move {
                AnalyticService::get_pattern_detection(task.sender, lr, ms, task.from, task.to)
                    .await;
            }
        });
    }

    fn consume_request(&mut self, req: types::RequestType) -> () {
        match req {
            RequestType::RunLearning => {
                if self.learning_handler.is_some() {
                    self.learning_handler.as_ref().unwrap().abort();
                    self.learning_handler = None;
                }
                self.learning_handler = Some(tokio::spawn({
                    self.learning_status = LearningStatus::Starting;
                    let tx = self.tx.clone();
                    let ms = self.metric_service.clone();
                    let ss = self.segments_service.clone();
                    async move {
                        AnalyticService::run_learning(tx, ms, ss).await;
                    }
                }));
            }
            RequestType::RunDetection(task) => {
                if self.learning_status == LearningStatus::Ready {
                    self.run_detection_task(task);
                } else {
                    self.learning_waiters.push(task);
                }
            }
            RequestType::GetStatus(tx) => {
                tx.send(self.learning_status.clone()).unwrap();
            }
        };
    }

    fn consume_response(&mut self, res: types::ResponseType) {
        match res {
            // TODO: handle when learning panic
            ResponseType::LearningStarted => self.learning_status = LearningStatus::Learning,
            ResponseType::LearningFinished(results) => {
                self.learning_handler = None;
                self.learning_results = Some(results);
                self.learning_status = LearningStatus::Ready;

                // TODO: run tasks from self.learning_waiter
                while self.learning_waiters.len() > 0 {
                    let task = self.learning_waiters.pop().unwrap();
                    self.run_detection_task(task);
                }
            }
            ResponseType::LearningFinishedEmpty => {
                self.learning_results = None;
                self.learning_status = LearningStatus::Initialization;
            }
        }
    }

    pub async fn serve(&mut self) {
        // TODO: remove this hack
        self.consume_request(RequestType::RunLearning);

        while let Some(message) = self.rx.recv().await {
            match message {
                AnalyticServiceMessage::Request(req) => self.consume_request(req),
                AnalyticServiceMessage::Response(res) => self.consume_response(res),
            }
        }
    }

    async fn run_learning(
        tx: mpsc::Sender<AnalyticServiceMessage>,
        ms: MetricService,
        ss: SegmentsService,
    ) {
        match tx
            .send(AnalyticServiceMessage::Response(
                ResponseType::LearningStarted,
            ))
            .await
        {
            Ok(_) => {}
            Err(_e) => println!("Fail to send notification about learning start"),
        }

        let prom = ms.get_prom();

        // TODO: logic for returning error

        // be careful if decide to store detections in db
        let segments = ss.get_segments_inside(0, u64::MAX / 2).unwrap();

        if segments.len() == 0 {
            match tx
                .send(AnalyticServiceMessage::Response(
                    ResponseType::LearningFinishedEmpty,
                ))
                .await
            {
                Ok(_) => {}
                Err(_e) => println!("Fail to send learning results"),
            }

            return;
        }

        let fs = segments
            .iter()
            .map(|s| prom.query(s.from, s.to, DETECTION_STEP));
        let rs = future::join_all(fs).await;

        // TODO: run this on label adding
        // TODO: save learning results in cache
        let mut learn_tss = Vec::new();
        for r in rs {
            let mr = r.unwrap();
            if mr.data.keys().len() == 0 {
                continue;
            }
            let k = mr.data.keys().nth(0).unwrap();
            let ts = &mr.data[k];
            // TODO: maybe not clone
            learn_tss.push(ts.clone());
        }

        let lr = PatternDetector::learn(&learn_tss).await;

        match tx
            .send(AnalyticServiceMessage::Response(
                ResponseType::LearningFinished(lr),
            ))
            .await
        {
            Ok(_) => {}
            Err(_e) => println!("Fail to send learning results"),
        }
    }

    async fn get_pattern_detection(
        tx: oneshot::Sender<anyhow::Result<Vec<Segment>>>,
        lr: LearningResults,
        ms: MetricService,
        from: u64,
        to: u64,
    ) {
        let prom = ms.get_prom();

        let pt = pattern_detector::PatternDetector::new(lr);
        let mr = prom.query(from, to, DETECTION_STEP).await.unwrap();

        if mr.data.keys().len() == 0 {
            match tx.send(Ok(Vec::new())) {
                Ok(_) => {}
                Err(_e) => {
                    println!("failed to send empty results");
                }
            }
            return;
        }

        let k = mr.data.keys().nth(0).unwrap();
        let ts = &mr.data[k];

        let result = pt.detect(ts);

        let result_segments: Vec<Segment> = result
            .iter()
            .map(|(p, q)| Segment {
                from: *p,
                to: *q,
                id: Some(utils::get_random_str(ID_LENGTH)),
                segment_type: SegmentType::Detection,
            })
            .collect();

        match tx.send(Ok(result_segments)) {
            Ok(_) => {}
            Err(_e) => {
                println!("failed to send results");
            }
        }
        return;
    }

    // TODO: move this to another analytic unit
    async fn get_threshold_detections(
        &self,
        from: u64,
        to: u64,
        step: u64,
        threashold: f64,
    ) -> anyhow::Result<Vec<Segment>> {
        let prom = self.metric_service.get_prom();
        let mr = prom.query(from, to, step).await?;

        if mr.data.keys().len() == 0 {
            return Ok(Vec::new());
        }

        let key = mr.data.keys().nth(0).unwrap();
        let ts = &mr.data[key];

        let mut result = Vec::<Segment>::new();
        let mut from: Option<u64> = None;
        for (t, v) in ts {
            if *v > threashold {
                if from.is_some() {
                    continue;
                } else {
                    from = Some(*t);
                }
            } else {
                if from.is_some() {
                    result.push(Segment {
                        // TODO: persist detections together with id
                        id: Some(get_random_str(ID_LENGTH)),
                        from: from.unwrap(),
                        to: *t,
                        segment_type: SegmentType::Detection,
                    });
                    from = None;
                }
            }
        }

        // TODO: don't repeat myself
        if from.is_some() {
            result.push(Segment {
                id: Some(get_random_str(ID_LENGTH)),
                from: from.unwrap(),
                to,
                segment_type: SegmentType::Detection,
            });
        }

        // TODO: decide what to do it from is Some() in the end

        Ok(result)
    }
}
