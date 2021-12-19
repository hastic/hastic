use crate::services::analytic_service::analytic_unit::types::AnalyticUnit;

use std::sync::Arc;

use chrono::Utc;

use tokio::sync::{mpsc, RwLock};

use super::types::{AnalyticUnitRF, DetectionRunnerConfig};
use tokio::time::{sleep, Duration};

pub struct DetectionRunner {
    config: DetectionRunnerConfig,
    analytic_unit: AnalyticUnitRF,
    running_handler: Option<tokio::task::JoinHandle<()>>,
}

impl DetectionRunner {
    pub fn new(config: DetectionRunnerConfig, analytic_unit: AnalyticUnitRF) -> DetectionRunner {
        DetectionRunner {
            config,
            analytic_unit,
            running_handler: None,
        }
    }

    pub fn run(&mut self, from: u64) {
        // TODO: get last detection timestamp from persistance
        // TODO: set last detection from "now"
        println!("Run begn");
        if self.running_handler.is_some() {
            self.running_handler.as_mut().unwrap().abort();
        }
        self.running_handler = Some(tokio::spawn({
            // TODO: clone channel
            let cfg = self.config.clone();
            async move {
                // AnalyticService::run_learning(tx, cfg, ms, ss).await;
                // TODO: run detection "from"
                // TODO: define window for detection
                // TODO: save last detection


                loop {
                    println!("runner detect");
                    // TODO: run detection periodically
                    sleep(Duration::from_secs(cfg.interval)).await;
                }
            }
        }));
    }

    pub async fn set_analytic_unit(&mut self, analytic_unit: AnalyticUnitRF,
    ) {
        self.analytic_unit = analytic_unit;
        // TODO: stop running_handler
        // TODO: rerun detection with new anomaly units
    }
}
