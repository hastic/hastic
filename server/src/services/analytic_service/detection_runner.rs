use crate::services::analytic_service::analytic_unit::types::{AnalyticUnit};

use std::sync::Arc;

use crate::config::Config;

use chrono::Utc;

use tokio::sync::{mpsc, RwLock};




struct DetectionRunner {
    config: Config,
    analytic_unit: Arc<RwLock<Box<dyn AnalyticUnit + Send + Sync>>>,
}

impl DetectionRunner {
    pub fn new(config: Config, analytic_unit: Arc<RwLock<Box<dyn AnalyticUnit + Send + Sync>>>) -> DetectionRunner {
        DetectionRunner { config, analytic_unit }
    }

    pub async fn run() {

    }
}
