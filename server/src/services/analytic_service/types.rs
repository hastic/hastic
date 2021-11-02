use crate::services::segments_service::Segment;

use super::pattern_detector::LearningResults;

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
pub enum RequestType {
    // TODO: convert to result RunLearning(anyhow::Result<()>)
    RunLearning,
    RunDetection(DetectionTask),
    GetStatus(oneshot::Sender<LearningStatus>),
}

#[derive(Debug)]
pub enum AnalyticServiceMessage {
    // Status,
    Request(RequestType),
    Response(ResponseType), // Detect { from: u64, to: u64 },
}
