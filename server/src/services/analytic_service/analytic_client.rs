use tokio::sync::mpsc;
use tokio::sync::oneshot;

use crate::services::segments_service::Segment;

use super::analytic_unit::types::AnalyticUnitConfig;
use super::analytic_unit::types::PatchConfig;
use super::types::DetectionTask;
use super::types::HSRTask;
use super::types::LearningStatus;

use super::types::HSR;
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
        self.tx.send(req).await?;
        let r = rx.await?;
        Ok(r)
    }

    pub async fn get_config(&self) -> anyhow::Result<AnalyticUnitConfig> {
        let (tx, rx) = oneshot::channel();
        let req = AnalyticServiceMessage::Request(RequestType::GetConfig(tx));
        self.tx.send(req).await?;
        let r = rx.await?;
        Ok(r)
    }

    pub async fn patch_config(&self, patch: PatchConfig) -> anyhow::Result<()> {
        let (tx, rx) = oneshot::channel();
        let req = AnalyticServiceMessage::Request(RequestType::PatchConfig(patch, tx));
        self.tx.send(req).await?;
        rx.await?;
        Ok(())
    }

    // pub async fn get_train(&self) -> anyhow::Result<LearningTrain> {
    //     let (tx, rx) = oneshot::channel();
    //     let req = AnalyticServiceMessage::Request(RequestType::GetLearningTrain(tx));
    //     self.tx.send(req).await?;
    //     let r = rx.await?;
    //     Ok(r)
    // }

    pub async fn get_pattern_detection(&self, from: u64, to: u64) -> anyhow::Result<Vec<Segment>> {
        let (tx, rx) = oneshot::channel();
        let req = AnalyticServiceMessage::Request(RequestType::RunDetection(DetectionTask {
            sender: tx,
            from,
            to,
        }));
        self.tx.send(req).await?;
        // TODO: handle second error
        match rx.await? {
            Ok(r) => Ok(r),
            Err(_) => Ok(Vec::new()),
        }
    }

    pub async fn get_hsr(&self, from: u64, to: u64) -> anyhow::Result<HSR> {
        let (tx, rx) = oneshot::channel();
        let req = AnalyticServiceMessage::Request(RequestType::GetHSR(HSRTask {
            sender: tx,
            from,
            to,
        }));
        self.tx.send(req).await?;
        // TODO: handle second error
        match rx.await? {
            Ok(r) => Ok(r),
            Err(_) => Ok(HSR::TimeSerie(Vec::new())),
        }
    }
}
