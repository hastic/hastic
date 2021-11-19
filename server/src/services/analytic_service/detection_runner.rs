use crate::services::analytic_service::analytic_unit::types::AnalyticUnit;

use std::sync::Arc;


use chrono::Utc;

use tokio::sync::{mpsc, RwLock};

use super::types::DetectionRunnerConfig;

pub struct DetectionRunner {
    config: DetectionRunnerConfig,
    analytic_unit: Arc<RwLock<Box<dyn AnalyticUnit + Send + Sync>>>,
}

impl DetectionRunner {
    pub fn new(
        config: DetectionRunnerConfig,
        analytic_unit: Arc<RwLock<Box<dyn AnalyticUnit + Send + Sync>>>,
    ) -> DetectionRunner {
        DetectionRunner {
            config,
            analytic_unit,
        }
    }

    pub async fn run() {
        // TODO: await detection step
        // TODO: get last detection timestamp from persistance
        // TODO: set lst detection from "now"
    }
}
