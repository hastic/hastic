use tokio::sync::mpsc;
use tokio::sync::oneshot;

use crate::services::segments_service::Segment;

use super::types::LearningStatus;
use super::types::{AnalyticServiceMessage, RequestType};

/// Client to be used multithreaded
#[derive(Clone)]
pub struct AnalyticClient {
    tx: mpsc::Sender<AnalyticServiceMessage>,
}

impl AnalyticClient {
    pub fn new(tx: mpsc::Sender<AnalyticServiceMessage>) -> AnalyticClient {
        AnalyticClient { tx }
    }

    pub async fn run_learning(&self) -> anyhow::Result<()> {
        self.tx
            .send(AnalyticServiceMessage::Request(RequestType::RunLearning))
            .await?;
        Ok(())
    }

    pub async fn get_status(&self) -> anyhow::Result<LearningStatus> {
        let (tx, rx) = oneshot::channel();
        let req = AnalyticServiceMessage::Request(RequestType::GetStatus(tx));
        let r = rx.await?;
        Ok(r)
    }

    pub async fn get_pattern_detection(&self, from: u64, to: u64) -> anyhow::Result<Vec<Segment>> {
        return Ok(Vec::new());
    }
}
