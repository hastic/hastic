use serde::{Deserialize, Serialize};

use async_trait::async_trait;

use crate::services::{
    analytic_service::types, metric_service::MetricService, segments_service::SegmentsService,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PatternConfig {
    pub correlation_score: f32,
    pub model_score: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnomalyConfig {
    pub sesonality: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThresholdConfig {
    pub threashold: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AnalyticUnitConfig {
    Pattern(PatternConfig),
    Threshold(ThresholdConfig),
}

pub enum LearningResult {
    Finished,
    FinishedEmpty,
    DatasourceError,
}

#[async_trait]
pub trait AnalyticUnit {
    async fn learn(&mut self, ms: MetricService, ss: SegmentsService) -> LearningResult;
    async fn detect(
        &self,
        ms: MetricService,
        from: u64,
        to: u64,
    ) -> anyhow::Result<Vec<(u64, u64)>>;
}
