use super::pattern_detector::LearningResults;

#[derive(Debug)]
pub enum ResponseType {
    LearningStarted,
    LearningFinished(LearningResults)
}

#[derive(Debug)]
pub enum RequestType {
    RunLearning
}

#[derive(Debug)]
pub enum AnalyticServiceMessage {
    // Status,
    Request(RequestType),
    Response(ResponseType)
    // Detect { from: u64, to: u64 },
}
