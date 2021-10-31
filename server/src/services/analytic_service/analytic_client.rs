use tokio::sync::mpsc;

use crate::services::segments_service::Segment;

use super::types::{AnalyticRequest};

/// CLient to be used multithreaded
///
///
#[derive(Clone)]
pub struct AnalyticClient {
    tx: mpsc::Sender<AnalyticRequest>,
}

impl AnalyticClient {
    pub fn new(tx: mpsc::Sender<AnalyticRequest>) -> AnalyticClient {
        AnalyticClient { tx }
    }
    pub async fn run_learning(&self) -> anyhow::Result<()> {
        self.tx.send(AnalyticRequest::RunLearning).await?;
        Ok(())
    }

    pub async fn get_pattern_detection(&self, from: u64, to: u64) -> anyhow::Result<Vec<Segment>> {
        return Ok(Vec::new());
    }
}
