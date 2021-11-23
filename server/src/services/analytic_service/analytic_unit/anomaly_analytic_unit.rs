use crate::services::{
    analytic_service::types::{HSR},
    metric_service::MetricService,
    segments_service::SegmentsService,
};

use super::types::{AnalyticUnit, AnalyticUnitConfig, AnomalyConfig, LearningResult};

use async_trait::async_trait;
use subbeat::metric::MetricResult;


struct SARIMA {
    pub ts: Vec<f64>,
    pub seasonality: u64
}

impl SARIMA {
    pub fn learn() {
        
    }
    pub fn predict(timestamp: u64, value: f64) -> (f64, f64, f64) {
        return (0.0, 0.0, 0.0);
    }
    // TODO: learn
    // TODO: update
    // TODO: predict with HSR
    // TODO: don't count NaNs in model
}


// TODO: move to config
const DETECTION_STEP: u64 = 10;

// offset from intex in timrange in seconds
fn get_value_with_offset(ts: &Vec<(u64, f64)>, index: usize, offset: u64) -> anyhow::Result<f64> {
    // TODO: implement
    if index == 0 {
        return Err(anyhow::format_err!("index should be > 0"));
    }
    return Ok(0.0);
    // let step =
    // let index_candidate =
    // let intex_candidate =
}

pub struct AnomalyAnalyticUnit {
    config: AnomalyConfig,
}

impl AnomalyAnalyticUnit {
    pub fn new(config: AnomalyConfig) -> AnomalyAnalyticUnit {
        AnomalyAnalyticUnit { config }
    }

    fn get_hsr_from_metric_result(&self, mr: &MetricResult) -> anyhow::Result<HSR> {
        if mr.data.keys().len() == 0 {
            return Ok(HSR::ConfidenceTimeSerie(Vec::new()));
        }

        let k = mr.data.keys().nth(0).unwrap();
        let ts = mr.data[k].clone();

        if ts.len() == 0 {
            return Ok(HSR::ConfidenceTimeSerie(Vec::new()));
        }

        let mut sts = Vec::new();
        sts.push((
            ts[0].0,
            ts[0].1,
            ((
                ts[0].1 + self.config.confidence,
                ts[0].1 - self.config.confidence,
            )),
        ));

        for t in 1..ts.len() {
            let alpha = self.config.alpha;
            let stv = alpha * ts[t].1 + (1.0 - alpha) * sts[t - 1].1;
            sts.push((
                ts[t].0,
                stv,
                (stv + self.config.confidence, stv - self.config.confidence),
            ));
        }

        Ok(HSR::ConfidenceTimeSerie(sts))
    }
}

#[async_trait]
impl AnalyticUnit for AnomalyAnalyticUnit {
    fn set_config(&mut self, config: AnalyticUnitConfig) {
        if let AnalyticUnitConfig::Anomaly(cfg) = config {
            self.config = cfg;
        } else {
            panic!("Bad config!");
        }
    }
    async fn learn(&mut self, _ms: MetricService, _ss: SegmentsService) -> LearningResult {
        // TODO: ensue that learning runs on seasonaliy change
        // TODO: load data to learning
        
        // TODO: update model to work online
        return LearningResult::Finished;
    }
    async fn detect(
        &self,
        ms: MetricService,
        from: u64,
        to: u64,
    ) -> anyhow::Result<Vec<(u64, u64)>> {
        let mr = ms
            .query(from - self.config.seasonality * 5, to, DETECTION_STEP)
            .await
            .unwrap();

        if mr.data.keys().len() == 0 {
            return Ok(Vec::new());
        }

        let k = mr.data.keys().nth(0).unwrap();
        let ts = mr.data[k].clone();

        if ts.len() == 0 {
            return Ok(Vec::new());
        }

        let mut result = Vec::new();

        let confidence_time_serie = self.get_hsr_from_metric_result(&mr)?;

        if let HSR::ConfidenceTimeSerie(hsr) = confidence_time_serie {
            let mut from = None;

            for ((t, _, (u, l)), (t1, rv)) in hsr.iter().zip(ts.iter()) {
                if *t != *t1 {
                    return Err(anyhow::format_err!("incompatible hsr/ts"));
                }
                if rv > u || rv < l {
                    if from.is_none() {
                        from = Some(*t);
                    }
                } else {
                    if from.is_some() {
                        result.push((from.unwrap(), *t));
                        from = None;
                    }
                }
            }

            if from.is_some() {
                result.push((from.unwrap(), ts.last().unwrap().0));
            }

            return Ok(result);
        } else {
            return Err(anyhow::format_err!("bad hsr"));
        }
    }

    // TODO: use hsr for learning and detections
    async fn get_hsr(&self, ms: MetricService, from: u64, to: u64) -> anyhow::Result<HSR> {
        let mr = ms.query(from, to, DETECTION_STEP).await.unwrap();
        return self.get_hsr_from_metric_result(&mr);
    }
}
