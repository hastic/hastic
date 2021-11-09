use crate::services::{
    analytic_service::types, metric_service::MetricService, segments_service::SegmentsService,
};

use super::types::{AnalyticUnit, AnomalyConfig, LearningResult};

use async_trait::async_trait;

// TODO: move to config
const DETECTION_STEP: u64 = 10;

pub struct AnomalyAnalyticUnit {
    config: AnomalyConfig,
}

impl AnomalyAnalyticUnit {
    pub fn new(config: AnomalyConfig) -> AnomalyAnalyticUnit {
        AnomalyAnalyticUnit { config }
    }
}

#[async_trait]
impl AnalyticUnit for AnomalyAnalyticUnit {
    async fn learn(&mut self, _ms: MetricService, _ss: SegmentsService) -> LearningResult {
        return LearningResult::Finished;
    }
    async fn detect(
        &self,
        ms: MetricService,
        from: u64,
        to: u64,
    ) -> anyhow::Result<Vec<(u64, u64)>> {
        let mr = ms.query(from, to, DETECTION_STEP).await.unwrap();

        if mr.data.keys().len() == 0 {
            return Ok(Vec::new());
        }

        let k = mr.data.keys().nth(0).unwrap();
        let ts = &mr.data[k];

        // TODO: implement
        // TODO: decide what to do it from is Some() in the end

        Ok(Default::default())
    }
}
