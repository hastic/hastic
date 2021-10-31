use super::pattern_detector::LearningResults;
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
}

#[derive(Debug)]
pub enum RequestType {
    RunLearning,
    GetStatus(oneshot::Sender<LearningStatus>),
}

#[derive(Debug)]
pub enum AnalyticServiceMessage {
    // Status,
    Request(RequestType),
    Response(ResponseType), // Detect { from: u64, to: u64 },
}
