use serde::{Deserialize, Serialize};

use async_trait::async_trait;

use crate::services::{
    analytic_service::types::HSR, metric_service::MetricService, segments_service::SegmentsService,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PatternConfig {
    pub correlation_score: f32,
    pub anti_correlation_score: f32,
    pub model_score: f32,
    pub threshold_score: f32,
}

impl Default for PatternConfig {
    fn default() -> Self {
        PatternConfig {
            correlation_score: 0.3,
            anti_correlation_score: 0.1,
            model_score: 0.8,
            threshold_score: 1.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnomalyConfig {
    pub alpha: f64,
    pub confidence: f64,
    pub seasonality: u64, // step in seconds, can be zero
    pub seasonality_iterations: u64,
}

impl Default for AnomalyConfig {
    fn default() -> Self {
        AnomalyConfig {
            alpha: 0.5,
            confidence: 10.0,
            seasonality: 60 * 60,
            seasonality_iterations: 3,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThresholdConfig {
    pub threshold: f64,
}

impl Default for ThresholdConfig {
    fn default() -> Self {
        ThresholdConfig { threshold: 0.5 }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AnalyticUnitConfig {
    Pattern(PatternConfig),
    Threshold(ThresholdConfig),
    Anomaly(AnomalyConfig),
}

impl AnalyticUnitConfig {
    // return true if need needs relearning
    pub fn patch(&self, patch: PatchConfig) -> (AnalyticUnitConfig, bool) {
        match patch {
            PatchConfig::Pattern(tcfg) => match self.clone() {
                AnalyticUnitConfig::Pattern(_) => {
                    if tcfg.is_some() {
                        return (AnalyticUnitConfig::Pattern(tcfg.unwrap()), false);
                    } else {
                        // TODO: it should be extraced from db
                        return (AnalyticUnitConfig::Pattern(Default::default()), false);
                    }
                }
                _ => {
                    if tcfg.is_some() {
                        return (AnalyticUnitConfig::Pattern(tcfg.unwrap()), true);
                    } else {
                        return (AnalyticUnitConfig::Pattern(Default::default()), true);
                    }
                }
            },

            PatchConfig::Anomaly(tcfg) => match self.clone() {
                AnalyticUnitConfig::Anomaly(scfg) => {
                    if tcfg.is_some() {
                        let t = tcfg.as_ref().unwrap();
                        let mut need_learning = t.seasonality != scfg.seasonality;
                        need_learning |= t.seasonality_iterations != scfg.seasonality_iterations;
                        return (AnalyticUnitConfig::Anomaly(tcfg.unwrap()), need_learning);
                    } else {
                        return (AnalyticUnitConfig::Anomaly(Default::default()), false);
                    }
                }
                _ => {
                    if tcfg.is_some() {
                        return (AnalyticUnitConfig::Anomaly(tcfg.unwrap()), true);
                    } else {
                        return (AnalyticUnitConfig::Anomaly(Default::default()), true);
                    }
                }
            },

            PatchConfig::Threshold(tcfg) => match self.clone() {
                AnalyticUnitConfig::Threshold(_) => {
                    if tcfg.is_some() {
                        return (AnalyticUnitConfig::Threshold(tcfg.unwrap()), false);
                    } else {
                        return (AnalyticUnitConfig::Threshold(Default::default()), false);
                    }
                }
                _ => {
                    if tcfg.is_some() {
                        return (AnalyticUnitConfig::Threshold(tcfg.unwrap()), true);
                    } else {
                        return (AnalyticUnitConfig::Threshold(Default::default()), true);
                    }
                }
            },
        }
    }
}

pub enum LearningResult {
    Finished,
    FinishedEmpty,
}

#[async_trait]
pub trait AnalyticUnit {
    fn get_id(&self) -> String;
    fn get_detection_window(&self) -> u64;
    async fn learn(
        &mut self,
        ms: MetricService,
        ss: SegmentsService,
    ) -> anyhow::Result<LearningResult>;
    async fn detect(
        &self,
        ms: MetricService,
        from: u64,
        to: u64,
    ) -> anyhow::Result<Vec<(u64, u64)>>;

    fn set_config(&mut self, c: AnalyticUnitConfig);
    async fn get_hsr(&self, ms: MetricService, from: u64, to: u64) -> anyhow::Result<HSR>;
}

#[derive(Deserialize, Serialize, Debug)]
pub enum PatchConfig {
    Pattern(Option<PatternConfig>),
    Threshold(Option<ThresholdConfig>),
    Anomaly(Option<AnomalyConfig>),
}

impl PatchConfig {
    pub fn get_type_id(&self) -> String {
        match &self {
            PatchConfig::Threshold(_) => "1".to_string(),
            PatchConfig::Pattern(_) => "2".to_string(),
            PatchConfig::Anomaly(_) => "3".to_string()
        }
    }
}