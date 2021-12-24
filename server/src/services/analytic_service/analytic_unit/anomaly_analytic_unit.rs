use crate::services::{
    analytic_service::types::{AnomalyHSRConfig, HSR},
    metric_service::MetricService,
    segments_service::SegmentsService,
};

use super::types::{AnalyticUnit, AnalyticUnitConfig, AnomalyConfig, LearningResult};

use async_trait::async_trait;
use subbeat::metric::MetricResult;

use chrono::prelude::*;

// TODO: move to config
const DETECTION_STEP: u64 = 10;

// timerange offset in seconds backwards from end of ts in assumption that ts has no gaps
fn get_value_with_offset(ts: &Vec<(u64, f64)>, offset: u64) -> Option<(u64, f64)> {
    // TODO: remove dependency to DETECTION_STEP

    let indexes_offset = (offset / DETECTION_STEP) as usize;
    let n = ts.len() - 1;
    if n < indexes_offset {
        return None;
    }
    let i = n - indexes_offset;
    return Some(ts[i]);
}

struct SARIMA {
    pub ts: Vec<(u64, f64)>,
    pub seasonality: u64,
    pub confidence: f64,
    pub seasonality_iterations: u64,
}

impl SARIMA {
    pub fn new(seasonality: u64, confidence: f64, seasonality_iterations: u64) -> SARIMA {
        return SARIMA {
            ts: Vec::new(),
            seasonality,
            confidence,
            seasonality_iterations,
        };
    }

    pub fn learn(&mut self, ts: &Vec<(u64, f64)>) -> anyhow::Result<()> {
        // TODO: don't count NaNs in model
        // TODO: add exponental smooting to the model
        // TODO: trend detection

        if ts.len() < 2 {
            return Err(anyhow::format_err!(
                "too short timeserie to learn from, timeserie length: {}",
                ts.len()
            ));
        }
        // TODO: ensure capacity with seasonality size
        let mut res_ts = Vec::<(u64, f64)>::new();
        let from = ts[0].0;
        // TODO: unwrap -> ?
        let to = ts.last().unwrap().0;
        let iter_steps = (self.seasonality / DETECTION_STEP) as usize;

        if to - from != self.seasonality_iterations * self.seasonality {
            return Err(anyhow::format_err!(
                "timeserie to learn from should be {} * sasonality",
                self.seasonality_iterations
            ));
        }

        for k in 0..iter_steps {
            let mut vts = Vec::new();
            for si in 0..self.seasonality_iterations {
                vts.push(ts[k + iter_steps * si as usize].1);
            }
            let mut vt: f64 = vts.iter().sum();
            vt /= self.seasonality_iterations as f64;
            let t = ts[ts.len() - iter_steps + k].0;
            res_ts.push((t, vt));
        }

        self.ts = res_ts;

        return Ok(());
    }
    pub fn predict(&self, mut timestamp: u64) -> (f64, (f64, f64)) {
        let from = self.ts[0].0;

        if timestamp < from {
            let len = from - timestamp;
            timestamp += self.seasonality * (len / self.seasonality);
            if len % self.seasonality != 0 {
                timestamp += self.seasonality;
            }
        }

        let len_from = timestamp - from;
        // TODO: take avg if timestamp in between
        let index_diff = (len_from / DETECTION_STEP) % self.ts.len() as u64;

        let p = self.ts[index_diff as usize].1;
        return (p, (p + self.confidence, p - self.confidence));
    }

    pub fn push_point() {
        // TODO: inmplement
    }
}

pub struct AnomalyAnalyticUnit {
    id: String,
    config: AnomalyConfig,
    sarima: Option<SARIMA>,
}

impl AnomalyAnalyticUnit {
    pub fn new(id: String, config: AnomalyConfig) -> AnomalyAnalyticUnit {
        AnomalyAnalyticUnit {
            id,
            config,
            sarima: None,
        }
    }

    fn get_hsr_from_metric_result(&self, mr: &MetricResult) -> anyhow::Result<HSR> {
        if self.sarima.is_none() {
            return Err(anyhow::format_err!("model is not ready"));
        }
        // TODO: get it from model
        if mr.data.keys().len() == 0 {
            return Ok(HSR::AnomalyHSR(AnomalyHSRConfig {
                seasonality: self.config.seasonality,
                timestamp: self.sarima.as_ref().unwrap().ts.last().unwrap().0,
                ts: Vec::new(),
            }));
        }

        let k = mr.data.keys().nth(0).unwrap();
        let ts = mr.data[k].clone();

        if ts.len() == 0 {
            return Ok(HSR::AnomalyHSR(AnomalyHSRConfig {
                seasonality: self.config.seasonality,
                timestamp: self.sarima.as_ref().unwrap().ts.last().unwrap().0,
                ts: Vec::new(),
            }));
        }

        let mut sts = Vec::new();
        let sarima = self.sarima.as_ref().unwrap();
        for vt in ts {
            let x = sarima.predict(vt.0);
            sts.push((vt.0, x.0, (x.1 .0, x.1 .1)));
        }

        return Ok(HSR::AnomalyHSR(AnomalyHSRConfig {
            seasonality: self.config.seasonality,
            timestamp: self.sarima.as_ref().unwrap().ts.last().unwrap().0,
            ts: sts,
        }));
    }
}

#[async_trait]
impl AnalyticUnit for AnomalyAnalyticUnit {
    fn get_id(&self) -> String {
        return self.id.to_owned();
    }
    fn get_detection_window(&self) -> u64 {
        // TODO: return window based on real petterns info
        return DETECTION_STEP;
    }
    fn set_config(&mut self, config: AnalyticUnitConfig) {
        if let AnalyticUnitConfig::Anomaly(cfg) = config {
            self.config = cfg;
            if self.sarima.is_some() {
                self.sarima.as_mut().unwrap().confidence = self.config.confidence;
            }
        } else {
            panic!("Bad config!");
        }
    }
    async fn learn(
        &mut self,
        ms: MetricService,
        _ss: SegmentsService,
    ) -> anyhow::Result<LearningResult> {
        let mut sarima = SARIMA::new(
            self.config.seasonality,
            self.config.confidence,
            self.config.seasonality_iterations,
        );

        let utc: DateTime<Utc> = Utc::now();
        let to = utc.timestamp() as u64;
        let from = to - self.config.seasonality * self.config.seasonality_iterations;

        let mr = ms.query(from, to, DETECTION_STEP).await?;
        if mr.data.keys().len() == 0 {
            return Ok(LearningResult::FinishedEmpty);
        }

        // TODO: unwrap -> ?
        let k = mr.data.keys().nth(0).unwrap();
        let ts = &mr.data[k];
        sarima.learn(ts)?;

        self.sarima = Some(sarima);

        // TODO: ensure that learning reruns on seasonaliy change
        // TODO: load data to learning

        // TODO: update model to work online
        return Ok(LearningResult::Finished);
    }
    async fn detect(
        &self,
        ms: MetricService,
        from: u64,
        to: u64,
    ) -> anyhow::Result<Vec<(u64, u64)>> {
        if self.sarima.is_none() {
            return Err(anyhow::format_err!("Learning model is not ready"));
        }
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

        if let HSR::AnomalyHSR(hsr) = confidence_time_serie {
            let mut from = None;

            for ((t, _, (u, l)), (t1, rv)) in hsr.ts.iter().zip(ts.iter()) {
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
