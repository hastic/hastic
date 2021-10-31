use crate::utils::{self, get_random_str};

use self::pattern_detector::{LearningResults, PatternDetector};

use super::{
    metric_service::MetricService,
    segments_service::{self, Segment, SegmentType, SegmentsService, ID_LENGTH},
};

use subbeat::metric::Metric;

use anyhow;

mod pattern_detector;

use futures::future;
use tokio::sync::oneshot;
use tokio::time::{sleep, Duration};

const DETECTION_STEP: u64 = 10;
const LEARNING_WAITING_INTERVAL: u64 = 100;

#[derive(Clone, PartialEq)]
enum LearningStatus {
    Initialization,
    Learning,
    Error,
    Ready,
}

#[derive(Clone)]
pub struct AnalyticService {
    metric_service: MetricService,
    segments_service: SegmentsService,
    learning_results: Option<LearningResults>,
    learning_status: LearningStatus,
}

impl AnalyticService {
    pub fn new(
        metric_service: MetricService,
        segments_service: segments_service::SegmentsService,
    ) -> AnalyticService {
        AnalyticService {
            metric_service,
            segments_service,
            // TODO: get it from persistance
            learning_results: None,
            learning_status: LearningStatus::Initialization,
        }
    }

    // call this from api
    pub async fn run_learning(&mut self) {
        let prom = self.metric_service.get_prom();
        let ss = self.segments_service.clone();

        let (tx, rx) = oneshot::channel();

        tokio::spawn(async move {
            // TODO: logic for returning error

            // be careful if decide to store detections in db
            let segments = ss.get_segments_inside(0, u64::MAX / 2).unwrap();

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

            if let Err(_) = tx.send(lr) {
                println!("Error: receive of learning results dropped");
            }
        });

        match rx.await {
            Ok(lr) => {
                self.learning_results = Some(lr);
                self.learning_status = LearningStatus::Ready;
            }
            Err(_) => {
                self.learning_status = LearningStatus::Error;
                println!("learning dropped")
            }
        }
    }

    pub async fn get_pattern_detection(&self, from: u64, to: u64) -> anyhow::Result<Vec<Segment>> {
        let prom = self.metric_service.get_prom();

        while self.learning_status == LearningStatus::Learning {
            sleep(Duration::from_millis(LEARNING_WAITING_INTERVAL)).await;
        }

        let lr = self.learning_results.as_ref().unwrap().clone();
        let pt = pattern_detector::PatternDetector::new(lr);
        let mr = prom.query(from, to, DETECTION_STEP).await?;

        if mr.data.keys().len() == 0 {
            return Ok(Vec::new());
        }

        let k = mr.data.keys().nth(0).unwrap();
        let ts = &mr.data[k];

        let result = pt.detect(ts);

        let result_segments = result
            .iter()
            .map(|(p, q)| Segment {
                from: *p,
                to: *q,
                id: Some(utils::get_random_str(ID_LENGTH)),
                segment_type: SegmentType::Detection,
            })
            .collect();

        // TODO: run detections
        // TODO: convert detections to segments
        Ok(result_segments)
    }

    pub async fn get_threshold_detections(
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
