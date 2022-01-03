use chrono::{DateTime, Utc};

use tokio::sync::mpsc;

use crate::services::metric_service::MetricService;

use super::types::{AnalyticServiceMessage, AnalyticUnitRF, DetectionRunnerConfig, ResponseType};
use tokio::time::{sleep, Duration};


pub struct DetectionRunner {
    tx: mpsc::Sender<AnalyticServiceMessage>,
    metric_service: MetricService,
    config: DetectionRunnerConfig,
    analytic_unit: AnalyticUnitRF,
    running_handler: Option<tokio::task::JoinHandle<()>>,
}

impl DetectionRunner {
    pub fn new(
        tx: mpsc::Sender<AnalyticServiceMessage>,
        metric_service: MetricService,
        config: DetectionRunnerConfig,
        analytic_unit: AnalyticUnitRF,
    ) -> DetectionRunner {
        DetectionRunner {
            metric_service,
            tx,
            config,
            analytic_unit,
            running_handler: None,
        }
    }

    pub fn run(&mut self, from: u64) {
        // TODO: set last detection from "now"
        if self.running_handler.is_some() {
            self.running_handler.as_mut().unwrap().abort();
        }
        self.running_handler = Some(tokio::spawn({
            let cfg = self.config.clone();
            let ms = self.metric_service.clone();
            let tx = self.tx.clone();
            let au = self.analytic_unit.clone();
            async move {
                // TODO: run detection "from" for big timespan
                // TODO: parse detections to webhooks
                // TODO: handle case when detection is in the end and continues after "now"
                //       it's better to make an issue on github

                let window_size = au.as_ref().read().await.get_detection_window();
                let detection_step = ms.get_detection_step();
                let mut t_from = from - window_size;
                let mut t_to = from;

                match tx
                    .send(AnalyticServiceMessage::Response(Ok(
                        ResponseType::DetectionRunnerStarted(from),
                    )))
                    .await
                {
                    Ok(_) => {}
                    Err(_e) => println!("Fail to send detection runner started notification"),
                }

                loop {
                    let a = au.as_ref().read().await;
                    let detections = a.detect(ms.clone(), t_from, t_to).await.unwrap();

                    for d in detections {
                        println!("detection: {} {}", d.0, d.1);
                    }

                    // TODO: send info about detections to tx

                    match tx
                        .send(AnalyticServiceMessage::Response(Ok(
                            ResponseType::DetectionRunnerUpdate(
                                au.as_ref().read().await.get_id(),
                                t_to,
                            ),
                        )))
                        .await
                    {
                        Ok(_) => {}
                        Err(_e) => println!("Fail to send detection runner started notification"),
                    }

                    sleep(Duration::from_secs(cfg.interval)).await;
                    t_from += detection_step;
                    t_to += detection_step;
                }
            }
        }));
    }

    // pub async fn set_analytic_unit(&mut self, analytic_unit: AnalyticUnitRF,
    // ) {
    //     self.analytic_unit = analytic_unit;
    //     // TODO: stop running_handler
    //     // TODO: rerun detection with new anomaly units
    // if self.runner_handler.is_some() {
    //     self.runner_handler.as_mut().unwrap().abort();
    // }
    // }
}
