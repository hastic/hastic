use crate::services::{
    analytic_service::types::HSR, metric_service::MetricService, segments_service::SegmentsService,
};

use super::types::{AnalyticUnit, AnalyticUnitConfig, LearningResult, ThresholdConfig};

use async_trait::async_trait;

// TODO: move to config
const DETECTION_STEP: u64 = 10;

pub struct ThresholdAnalyticUnit {
    id: String,
    config: ThresholdConfig,
}

impl ThresholdAnalyticUnit {
    pub fn new(id: String, config: ThresholdConfig) -> ThresholdAnalyticUnit {
        ThresholdAnalyticUnit { id, config }
    }
}

#[async_trait]
impl AnalyticUnit for ThresholdAnalyticUnit {
    fn get_id(&self) -> String {
        return self.id.to_owned();
    }
    fn get_detection_window(&self) -> u64 {
        return DETECTION_STEP;
    }
    async fn learn(
        &mut self,
        _ms: MetricService,
        _ss: SegmentsService,
    ) -> anyhow::Result<LearningResult> {
        return Ok(LearningResult::Finished);
    }

    fn set_config(&mut self, config: AnalyticUnitConfig) {
        if let AnalyticUnitConfig::Threshold(cfg) = config {
            self.config = cfg;
        } else {
            panic!("Bad config!");
        }
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

        let mut result = Vec::<(u64, u64)>::new();
        let mut from: Option<u64> = None;
        for (t, v) in ts {
            if *v > self.config.threshold {
                if from.is_some() {
                    continue;
                } else {
                    from = Some(*t);
                }
            } else {
                if from.is_some() {
                    result.push((from.unwrap(), *t));
                    from = None;
                }
            }
        }

        // TODO: don't repeat myself
        if from.is_some() {
            result.push((from.unwrap(), ts.last().unwrap().0));
        }

        // TODO: decide what to do it from is Some() in the end

        Ok(result)
    }

    // TODO: use hsr for learning and detections
    async fn get_hsr(&self, ms: MetricService, from: u64, to: u64) -> anyhow::Result<HSR> {
        let mr = ms.query(from, to, DETECTION_STEP).await.unwrap();

        if mr.data.keys().len() == 0 {
            return Ok(HSR::TimeSerie(Vec::new()));
        }

        let k = mr.data.keys().nth(0).unwrap();
        let ts = mr.data[k].clone();

        Ok(HSR::TimeSerie(ts))
    }
}
