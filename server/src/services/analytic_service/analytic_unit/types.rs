use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PatternConfig {
    pub correlation_score: f32,
    pub model_score: f32
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThresholdConfig {
    pub threashold: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AnalyticUnitConfig {
    Pattern(PatternConfig),
    Threshold(ThresholdConfig)
}