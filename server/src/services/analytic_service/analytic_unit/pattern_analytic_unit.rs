use std::{fmt, sync::Arc};

use futures::future;
use parking_lot::Mutex;

use linfa::prelude::*;

use linfa;
use linfa_svm::Svm;

use ndarray::Array;

use crate::services::{
    analytic_service::types::{self, LearningTrain},
    metric_service::MetricService,
    segments_service::{Segment, SegmentType, SegmentsService},
};

use super::types::{AnalyticUnit, AnalyticUnitConfig, LearningResult, PatternConfig};

use async_trait::async_trait;

// TODO: move to config
const DETECTION_STEP: u64 = 10;

#[derive(Clone)]
pub struct LearningResults {
    model: Arc<Mutex<Svm<f64, bool>>>,

    pub learning_train: LearningTrain,

    patterns: Vec<Vec<f64>>,
    anti_patterns: Vec<Vec<f64>>,
}

// impl Clone for LearningResults {
//     fn clone(&self) -> Self {
//         // TODO: it's a hack
//         // https://github.com/rust-ml/linfa/issues/174
//         let model_str = serde_json::to_string(&self.model).unwrap();
//         let model = serde_json::from_str(&model_str).unwrap();
//         return LearningResults {
//             model,
//             patterns: self.patterns.clone(),
//             anti_patterns: self.anti_patterns.clone()
//         };
//     }
// }

impl fmt::Debug for LearningResults {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("{:?}", &self.patterns)
            .field("{:?}", &self.anti_patterns)
            .finish()
    }
}

pub const FEATURES_SIZE: usize = 4;

pub type Features = [f64; FEATURES_SIZE];

fn nan_to_zero(n: f64) -> f64 {
    if n.is_nan() {
        return 0.;
    }
    return n;
}

struct SegData {
    label: bool,
    data: Vec<(u64, f64)>,
}

async fn segment_to_segdata(ms: &MetricService, segment: &Segment) -> anyhow::Result<SegData> {
    let mut mr = ms.query(segment.from, segment.to, DETECTION_STEP).await?;

    if mr.data.keys().len() == 0 {
        return Ok(SegData {
            label: segment.segment_type == SegmentType::Label,
            data: Default::default(),
        });
    }

    let k = mr.data.keys().nth(0).unwrap().clone();
    let ts = mr.data.remove(&k).unwrap();

    Ok(SegData {
        label: segment.segment_type == SegmentType::Label,
        data: ts,
    })
}

pub struct PatternAnalyticUnit {
    config: PatternConfig,
    learning_results: Option<LearningResults>,
}

// TODO: move this to loginc of analytic unit
impl PatternAnalyticUnit {
    pub fn new(cfg: PatternConfig) -> PatternAnalyticUnit {
        PatternAnalyticUnit {
            config: cfg,
            learning_results: None,
        }
    }

    fn corr_aligned(xs: &Vec<f64>, ys: &Vec<f64>) -> f32 {
        let n = xs.len() as f64;
        let mut s_xs: f64 = 0f64;
        let mut s_ys: f64 = 0f64;
        let mut s_xsys: f64 = 0f64;
        let mut s_xs_2: f64 = 0f64;
        let mut s_ys_2: f64 = 0f64;

        let min = xs.len().min(ys.len());
        xs.iter()
            .take(min)
            .zip(ys.iter().take(min))
            .for_each(|(xi, yi)| {
                s_xs += xi;
                s_ys += yi;
                s_xsys += xi * yi;
                s_xs_2 += xi * xi;
                s_ys_2 += yi * yi;
            });

        let numerator: f64 = n * s_xsys - s_xs * s_ys;
        let denominator: f64 = ((n * s_xs_2 - s_xs * s_xs) * (n * s_ys_2 - s_ys * s_ys)).sqrt();

        // IT"s a hack
        if denominator < 0.01 {
            return 0.;
        }

        let result: f64 = numerator / denominator;

        // assert!(result.abs() <= 1.01);

        if result.abs() > 1.1 {
            println!("{:?}", xs);
            println!("------------");
            println!("{:?}", ys);
            println!("WARNING: corr result > 1: {}", result);
        }

        return result as f32; // we know that it's in -1..1
    }

    fn get_features(xs: &Vec<f64>) -> Features {
        let mut min = f64::MAX;
        let mut max = f64::MIN;
        let mut sum = 0f64;

        for x in xs {
            min = min.min(*x);
            max = max.max(*x);
            sum += x;
        }

        let mean = sum / xs.len() as f64;

        sum = 0f64;

        for x in xs {
            sum += (x - mean) * (x - mean);
        }

        let sd = sum.sqrt();

        // TODO: add autocorrelation
        // TODO: add FFT
        // TODO: add DWT

        return [
            min, max, mean, sd,
            // 0f64,0f64,
            // 0f64,0f64,0f64, 0f64
        ];
    }
}

#[async_trait]
impl AnalyticUnit for PatternAnalyticUnit {

    fn set_config(&mut self, config: AnalyticUnitConfig) {
        if let AnalyticUnitConfig::Pattern(cfg) = config {
            self.config = cfg;
        } else {
            panic!("Bad config!");
        }
    }

    async fn learn(&mut self, ms: MetricService, ss: SegmentsService) -> LearningResult {
        // be careful if decide to store detections in db
        let segments = ss.get_segments_inside(0, u64::MAX / 2).unwrap();
        let has_segments_label = segments
            .iter()
            .find(|s| s.segment_type == SegmentType::Label)
            .is_some();

        if !has_segments_label {
            return LearningResult::FinishedEmpty;
        }

        let fs = segments.iter().map(|s| segment_to_segdata(&ms, s));
        let rs = future::join_all(fs).await;

        let mut learn_tss = Vec::new();
        let mut learn_anti_tss = Vec::new();

        for r in rs {
            if r.is_err() {
                println!("Error extracting metrics from datasource");
                return LearningResult::DatasourceError;
            }

            let sd = r.unwrap();
            if sd.data.is_empty() {
                continue;
            }
            if sd.label {
                learn_tss.push(sd.data);
            } else {
                learn_anti_tss.push(sd.data);
            }
        }

        // let reads: &Vec<Vec<(u64, f64)>> = // TODO
        // let anti_reads: &Vec<Vec<(u64, f64)>> // TODO

        // let size_avg = reads.iter().map(|r| r.len()).sum::<usize>() / reads.len();

        let mut patterns = Vec::<Vec<f64>>::new();
        let mut anti_patterns = Vec::<Vec<f64>>::new();

        let mut records_raw = Vec::<Features>::new();
        let mut targets_raw = Vec::<bool>::new();

        for r in learn_tss {
            let xs: Vec<f64> = r.iter().map(|e| e.1).map(nan_to_zero).collect();
            let fs = PatternAnalyticUnit::get_features(&xs);

            records_raw.push(fs);
            targets_raw.push(true);
            patterns.push(xs);
        }

        for r in learn_anti_tss {
            let xs: Vec<f64> = r.iter().map(|e| e.1).map(nan_to_zero).collect();
            let fs = PatternAnalyticUnit::get_features(&xs);
            records_raw.push(fs);
            targets_raw.push(false);
            anti_patterns.push(xs);
        }

        let records = Array::from_shape_fn((records_raw.len(), FEATURES_SIZE), |(i, j)| {
            records_raw[i][j]
        });

        let targets = Array::from_vec(targets_raw.clone());

        // println!("{:?}", records);
        // println!("{:?}", targets);

        let train = linfa::Dataset::new(records, targets);

        // The 'view' describes what set of data is drawn
        // let v = ContinuousView::new()
        //     .add(s1)
        //     // .add(s2)
        //     .x_range(-500., 100.)
        //     .y_range(-200., 600.)
        //     .x_label("Some varying variable")
        //     .y_label("The response of something");

        // Page::single(&v).save("scatter.svg").unwrap();

        // let model = stat.iter().map(|(c, v)| v / *c as f64).collect();

        let model = Svm::<_, bool>::params()
            .pos_neg_weights(50000., 5000.)
            .gaussian_kernel(80.0)
            .fit(&train)
            .unwrap();

        // let prediction = model.predict(Array::from_vec(vec![
        //     715.3122807017543, 761.1228070175438, 745.0, 56.135764727158595, 0.0, 0.0
        // ]));

        // println!("pridiction: {}", prediction );

        self.learning_results = Some(LearningResults {
            model: Arc::new(Mutex::new(model)),

            learning_train: LearningTrain {
                features: records_raw,
                target: targets_raw,
            },

            patterns,
            anti_patterns,
        });

        return LearningResult::Finished;
    }

    // TODO: get iterator instead of vector
    async fn detect(
        &self,
        ms: MetricService,
        from: u64,
        to: u64,
    ) -> anyhow::Result<Vec<(u64, u64)>> {
        if self.learning_results.is_none() {
            return Err(anyhow::format_err!("Learning results are not ready"));
        }

        let mr = ms.query(from, to, DETECTION_STEP).await.unwrap();

        if mr.data.keys().len() == 0 {
            return Ok(Vec::new());
        }

        let k = mr.data.keys().nth(0).unwrap();
        let ts = &mr.data[k];

        let lr = self.learning_results.as_ref().unwrap();
        let mut results = Vec::new();

        let pt = &lr.patterns;
        let apt = &lr.anti_patterns;

        for i in 0..ts.len() {
            let mut pattern_match_score = 0f32;
            let mut pattern_match_len = 0usize;
            let mut anti_pattern_match_score = 0f32;

            for p in pt {
                if i + p.len() < ts.len() {
                    let mut backet = Vec::<f64>::new();
                    for j in 0..p.len() {
                        backet.push(nan_to_zero(ts[i + j].1));
                    }
                    let score = PatternAnalyticUnit::corr_aligned(&p, &backet);
                    if score > pattern_match_score {
                        pattern_match_score = score;
                        pattern_match_len = p.len();
                    }
                }
            }

            for p in apt {
                if i + p.len() < ts.len() {
                    let mut backet = Vec::<f64>::new();
                    for j in 0..p.len() {
                        backet.push(nan_to_zero(ts[i + j].1));
                    }
                    let score = PatternAnalyticUnit::corr_aligned(&p, &backet);
                    if score > anti_pattern_match_score {
                        anti_pattern_match_score = score;
                    }
                }
            }

            {
                let mut backet = Vec::<f64>::new();
                for j in 0..pattern_match_len {
                    backet.push(nan_to_zero(ts[i + j].1));
                }
                let fs = PatternAnalyticUnit::get_features(&backet);
                let detected = lr.model.lock().predict(Array::from_vec(fs.to_vec()));
                if detected {
                    pattern_match_score += self.config.model_score;
                }
            }

            if pattern_match_score - anti_pattern_match_score * self.config.anti_correlation_score
                >= self.config.threshold_score
            {
                results.push((ts[i].0, ts[i + pattern_match_len - 1].0));
            }
        }

        Ok(results)
    }
}
