use chrono::{Utc, DateTime};

use tokio::sync::{mpsc};

use super::types::{AnalyticServiceMessage, AnalyticUnitRF, DetectionRunnerConfig, ResponseType};
use tokio::time::{sleep, Duration};


const DETECTION_STEP: u64 = 10;


pub struct DetectionRunner {
    tx: mpsc::Sender<AnalyticServiceMessage>,
    config: DetectionRunnerConfig,
    analytic_unit: AnalyticUnitRF,
    running_handler: Option<tokio::task::JoinHandle<()>>,
}

impl DetectionRunner {
    pub fn new(
        tx: mpsc::Sender<AnalyticServiceMessage>,
        config: DetectionRunnerConfig,
        analytic_unit: AnalyticUnitRF,
    ) -> DetectionRunner {
        DetectionRunner {
            tx,
            config,
            analytic_unit,
            running_handler: None,
        }
    }

    pub fn run(&mut self, from: u64) {
        // TODO: get last detection timestamp from persistance
        // TODO: set last detection from "now"
        if self.running_handler.is_some() {
            self.running_handler.as_mut().unwrap().abort();
        }
        self.running_handler = Some(tokio::spawn({
            // TODO: clone channel
            let cfg = self.config.clone();
            let tx = self.tx.clone();
            let au = self.analytic_unit.clone();
            async move {
                // TODO: run detection "from" for big timespan
                // TODO: parse detections to webhooks
                // TODO: define window for detection
                // TODO: handle case when detection is in the end and continues after "now"

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
                    // TODO: don't use DateTime, but count timestamp by steps
                    let now: DateTime<Utc> = Utc::now();
                    let to = now.timestamp() as u64;
                    // TODO: run detection periodically
                    sleep(Duration::from_secs(cfg.interval)).await;
                    // TODO: update from / to based on detection step
                    match tx.send(AnalyticServiceMessage::Response(Ok(
                        ResponseType::DetectionRunnerUpdate(au.as_ref().read().await.get_id(), to)
                    ))).await {
                        Ok(_) => {},
                        Err(_e) => println!("Fail to send detection runner started notification"),
                    }
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
