use std::{collections::VecDeque, fmt, sync::Arc};

use futures::future;
use parking_lot::Mutex;


use gbdt::config::Config;
use gbdt::decision_tree::{Data, DataVec, PredVec};
use gbdt::gradient_boost::GBDT;


use crate::services::{analytic_service::types::{self, HSR, LearningTrain}, metric_service::MetricService, segments_service::{Segment, SegmentType, SegmentsService}};

use super::types::{AnalyticUnit, AnalyticUnitConfig, LearningResult, PatternConfig};

use async_trait::async_trait;

use rustfft::{self, FftPlanner, num_complex::Complex};

// TODO: move to config
const DETECTION_STEP: u64 = 10;
const FFT_LEN: usize = 64;

// TODO: convert to vector
pub const FEATURES_SIZE: usize = 4 + 16 * 2;
pub type Features = Vec<f64>;

#[derive(Clone)]
pub struct LearningResults {
    // TODO: replace with RWLock
    model: Arc<Mutex<GBDT>>,

    pub learning_train: LearningTrain,

    patterns: Vec<Vec<f64>>,
    anti_patterns: Vec<Vec<f64>>,

    avg_pattern_length: usize,
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

    // TODO: add DWT

    let mut planner = FftPlanner::<f64>::new();

    // TODO: move 128 to config
    let fft = planner.plan_fft_forward(FFT_LEN);
    let mut c_buffer = vec![Complex{ re: 0.0f64, im: 0.0f64 }; FFT_LEN];

    for i in 0..FFT_LEN.min(xs.len()) {
        c_buffer[i].re = xs[i];
    }

    fft.process(&mut c_buffer);

    // https://docs.rs/rustfft/6.0.1/rustfft/index.html#normalization
    let norm_factor = (FFT_LEN.min(xs.len()) as f64).sqrt();
    for i in 0..FFT_LEN.min(xs.len()) {
        c_buffer[i] /= norm_factor;
    }

    return vec![
        min, max, 
        mean, sd,
        c_buffer[0].re, c_buffer[0].im,
        c_buffer[1].re, c_buffer[1].im,
        c_buffer[2].re, c_buffer[2].im,
        c_buffer[3].re, c_buffer[3].im,
        c_buffer[4].re, c_buffer[4].im,
        c_buffer[5].re, c_buffer[5].im,
        c_buffer[6].re, c_buffer[6].im,
        c_buffer[7].re, c_buffer[7].im,
        c_buffer[8].re, c_buffer[8].im,
        c_buffer[9].re, c_buffer[9].im,
        c_buffer[10].re, c_buffer[10].im,
        c_buffer[11].re, c_buffer[11].im,
        c_buffer[12].re, c_buffer[12].im,
        c_buffer[13].re, c_buffer[13].im,
        c_buffer[14].re, c_buffer[14].im,
        c_buffer[15].re, c_buffer[15].im,
        // 0f64,0f64,
        // 0f64,0f64,0f64, 0f64
    ];
}

fn corr_aligned(xs: &VecDeque<f64>, ys: &Vec<f64>) -> f32 {
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

fn max_corr_with_segments(xs: &VecDeque<f64>, yss: &Vec<Vec<f64>>) -> f32 {
    let mut max_corr = 0.0; // we just take positive part of correlation
    for ys in yss.iter() {
        let c = corr_aligned(xs, ys);
        // TODO: check that here no NaNs
        if c > max_corr {
            max_corr = c;
        }
    }
    return max_corr;
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

        // TODO: move to config
        let mut cfg = Config::new();
        cfg.set_feature_size(FEATURES_SIZE);
        cfg.set_max_depth(3);
        cfg.set_iterations(50);
        cfg.set_shrinkage(0.1);
        cfg.set_loss("LogLikelyhood"); 
        cfg.set_debug(false);
        cfg.set_data_sample_ratio(1.0);
        cfg.set_feature_sample_ratio(1.0);
        cfg.set_training_optimization_level(2);

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

        let mut patterns = Vec::<Vec<f64>>::new();
        let mut anti_patterns = Vec::<Vec<f64>>::new();

        let mut records_raw = Vec::<Features>::new();
        let mut targets_raw = Vec::<bool>::new();

        let mut pattern_length_size_sum = 0usize;

        for r in learn_tss {
            let xs: Vec<f64> = r.iter().map(|e| e.1).map(nan_to_zero).collect();
            let fs = get_features(&xs);

            records_raw.push(fs);
            targets_raw.push(true);
            pattern_length_size_sum += xs.len();
            patterns.push(xs);
        }

        for r in learn_anti_tss {
            let xs: Vec<f64> = r.iter().map(|e| e.1).map(nan_to_zero).collect();
            let fs = get_features(&xs);
            records_raw.push(fs);
            targets_raw.push(false);
            pattern_length_size_sum += xs.len();
            anti_patterns.push(xs);
        }

        let mut train_dv = Vec::new();
        assert_eq!(records_raw.len(), targets_raw.len());

        for i in 0..records_raw.len() {
            let data = Data::new_training_data(
                records_raw[i].iter().map(|e| *e as f32).collect(),
                1.0,
                if targets_raw[i] { 1.0 } else { -1.0 },
                Some(0.5)
            );
            // println!("{:?}", targets_raw[i]);
            train_dv.push(data);
        }


        let mut model = GBDT::new(&cfg);
        model.fit(&mut train_dv);

        let avg_pattern_length = pattern_length_size_sum / (&patterns.len() + &anti_patterns.len());

        self.learning_results = Some(LearningResults {
            model: Arc::new(Mutex::new(model)),

            learning_train: LearningTrain {
                features: records_raw,
                target: targets_raw,
            },

            patterns,
            anti_patterns,

            avg_pattern_length,
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

        if lr.avg_pattern_length > ts.len() {
            // TODO: handle case when we inside pattern
            return Ok(results);
        }

        let mut window = VecDeque::<f64>::new();
        for i in 0..lr.avg_pattern_length {
            window.push_back(nan_to_zero(ts[i].1));
        }

        let mut i = lr.avg_pattern_length - 1;

        let mut from: Option<u64> = None;
        let mut to: Option<u64> = None;

        loop {
            let positive_corr = max_corr_with_segments(&window, pt);
            let negative_corr = max_corr_with_segments(&window, apt);

            let model_weight = {
                let mut vs: Vec<f64> = Vec::new();
                for v in window.iter() {
                    vs.push(*v);
                }
                let fs = get_features(&vs).iter().map(|e| *e as f32).collect();
                let lk = lr.model.lock();

                let data_t1 = Data::new_test_data(fs, Some(0.5));
                let mut test_dv = Vec::new();
                test_dv.push(data_t1);
                lk.predict(&test_dv)[0]
            };

            let score = positive_corr * self.config.correlation_score
                - negative_corr * self.config.anti_correlation_score
                + model_weight as f32 * self.config.model_score;

            // TODO: replace it with score > config.score_treshold
            if score > self.config.threshold_score {
                // inside pattern
                if from.is_none() {
                    from = Some(ts[i - (lr.avg_pattern_length - 1)].0);
                }
                to = Some(ts[i].0);
            } else {
                if to.is_some() {
                    // merge with last
                    if results.len() > 0 && results.last().unwrap().1 >= from.unwrap() {
                        let (prev_from, _) = results.pop().unwrap();
                        results.push((prev_from, to.unwrap()));
                    } else {
                        results.push((from.unwrap(), to.unwrap()));
                    }
                    from = None;
                    to = None;
                }
            }

            i += 1;
            if i == ts.len() {
                break;
            }

            window.pop_front();
            window.push_back(ts[i].1);
        }

        if to.is_some() {
            results.push((from.unwrap(), to.unwrap()));
            from = None;
            to = None;
        }

        Ok(results)
    }

    // TODO: use hsr for learning and detections
    async fn get_hsr(
        &self,
        ms: MetricService,
        from: u64,
        to: u64,
    ) -> anyhow::Result<HSR> {
        let mr = ms.query(from, to, DETECTION_STEP).await.unwrap();

        if mr.data.keys().len() == 0 {
            return Ok(HSR::TimeSerie(Vec::new()));
        }

        let k = mr.data.keys().nth(0).unwrap();
        let ts = mr.data[k].clone();

        Ok(HSR::TimeSerie(ts))
    }
}
