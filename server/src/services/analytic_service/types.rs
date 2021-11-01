use crate::services::segments_service::Segment;

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
    LearningFinishedEmpty
}

#[derive(Debug)]
pub struct DetectionTask {
    pub sender: oneshot::Sender<LearningStatus>,
    pub from: u64, 
    pub to: u64
}

#[derive(Debug)]
pub enum RequestType {
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
