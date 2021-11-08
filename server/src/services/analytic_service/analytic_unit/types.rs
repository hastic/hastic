use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PatternDetectorConfig {
    pub correlation_score: f32,
    pub model_score: f32
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AnalyticUnitConfig {
    PatternDetector(PatternDetectorConfig)
}