use tokio::sync::oneshot;

#[derive(Debug, PartialEq)]
pub enum AnalyticRequest {
    // Status,
    RunLearning,
    // Detect { from: u64, to: u64 },
}
