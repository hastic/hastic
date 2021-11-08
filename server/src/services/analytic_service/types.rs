use crate::services::segments_service::Segment;

use super::analytic_unit::{pattern_detector::{self, LearningResults}, types::AnalyticUnitConfig};

use anyhow::Result;
use serde::Serialize;
use tokio::sync::oneshot;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum LearningStatus {
    Initialization,
    Starting,
    Learning,
    Error,
    Ready,
}

// TODO: move to analytic_unit config of pattern detector
#[derive(Clone, Serialize, Debug)]
pub struct LearningTrain {
    pub features: Vec<pattern_detector::Features>,
    pub target: Vec<bool>,
}

impl Default for LearningTrain {
    fn default() -> Self {
        return LearningTrain {
            features: Vec::new(),
            target: Vec::new(),
        };
    }
}

#[derive(Debug)]
pub enum ResponseType {
    LearningStarted,
    LearningFinished(LearningResults),
    LearningFinishedEmpty,
    LearningDatasourceError,
}

#[derive(Debug)]
pub struct DetectionTask {
    pub sender: oneshot::Sender<Result<Vec<Segment>>>,
    pub from: u64,
    pub to: u64,
}

#[derive(Debug)]
pub struct DetectionRunnerConfig {
    // pub sender: mpsc::Sender<Result<Vec<Segment>>>,
    pub endpoint: String,
    pub from: u64,
}

#[derive(Debug)]
pub enum RequestType {
    // TODO: convert to result RunLearning(anyhow::Result<()>)
    RunLearning,
    RunDetection(DetectionTask),
    GetStatus(oneshot::Sender<LearningStatus>),
    GetConfig(oneshot::Sender<AnalyticUnitConfig>),
    GetLearningTrain(oneshot::Sender<LearningTrain>),
}

#[derive(Debug)]
pub enum AnalyticServiceMessage {
    // Status,
    Request(RequestType),
    Response(ResponseType), // Detect { from: u64, to: u64 },
}
