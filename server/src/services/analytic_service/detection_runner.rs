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
    pub fn new(
        config: DetectionRunnerConfig,
        analytic_unit: AnalyticUnitRF,
    ) -> DetectionRunner {
        DetectionRunner {
            config,
            analytic_unit,
            running_handler: None
        }
    }

    pub async fn run(&mut self) {
        
        // TODO: get last detection timestamp from persistance
        // TODO: set lst detection from "now"
        if self.running_handler.is_some() {
            self.running_handler.as_mut().unwrap().abort();
        }
        self.running_handler = Some(tokio::spawn({
            // TODO: clone channel
            async move {
                // AnalyticService::run_learning(tx, cfg, ms, ss).await;
                // TODO: run detection
                // TODO: await detection step

                loop {
                    // TODO: use interval
                    sleep(Duration::from_secs(100)).await;
                }

            }
        }));

    }

    pub async fn set_analytic_unit(analytic_unit: Arc<RwLock<Box<dyn AnalyticUnit + Send + Sync>>>) {
        
    }
}
