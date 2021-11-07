
use std::{fmt, sync::Arc};


use parking_lot::Mutex;
use serde_json;
use serde::{Serialize, Deserialize};

use linfa::prelude::*;

use linfa;
use linfa_svm::{error::Result, Svm};

use ndarray::{Array, ArrayView, Axis};



#[derive(Clone)]
pub struct LearningResults {
    model: Arc<Mutex<Svm<f64, bool>>>,
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


const FEATURES_SIZE: usize = 6;

type Features = [f64; FEATURES_SIZE];

const SCORE_THRESHOLD: f64 = 0.95;

#[derive(Clone)]
pub struct PatternDetector {
    learning_results: LearningResults,
}

fn nan_to_zero(n: f64) -> f64 {
    if n.is_nan() {
        return 0.;
    }
    return n;
}

// TODO: move this to loginc of analytic unit
impl PatternDetector {
    pub fn new(learning_results: LearningResults) -> PatternDetector {
        PatternDetector { learning_results }
    }

    pub async fn learn(
        reads: &Vec<Vec<(u64, f64)>>,
        anti_reads: &Vec<Vec<(u64, f64)>>,
    ) -> LearningResults {
        // let size_avg = reads.iter().map(|r| r.len()).sum::<usize>() / reads.len();

        let mut patterns = Vec::<Vec<f64>>::new();
        let mut anti_patterns = Vec::<Vec<f64>>::new();

        
        let mut records = Array::zeros((0, FEATURES_SIZE));
        let mut targets_raw = Vec::<bool>::new();

        for r in reads {
            let xs: Vec<f64> = r.iter().map(|e| e.1).map(nan_to_zero).collect();
            let fs = PatternDetector::get_features(&xs);
            
            records.push_row(ArrayView::from(&fs)).unwrap();
            
            targets_raw.push(true);
            patterns.push(xs);
        }

        for r in anti_reads {
            let xs: Vec<f64> = r.iter().map(|e| e.1).map(nan_to_zero).collect();
            let fs = PatternDetector::get_features(&xs);
            records.push_row(ArrayView::from(&fs)).unwrap();
            targets_raw.push(false);
            anti_patterns.push(xs);
        }

        let targets = Array::from_vec(targets_raw);

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
            .fit(&train).unwrap();
        
        
        // let prediction = model.predict(Array::from_vec(vec![
        //     715.3122807017543, 761.1228070175438, 745.0, 56.135764727158595, 0.0, 0.0
        // ]));

        // println!("pridiction: {}", prediction );

        LearningResults {
            model: Arc::new(Mutex::new(model)),
            patterns,
            anti_patterns,
        }
    }

    // TODO: get iterator instead of vector
    pub fn detect(&self, ts: &Vec<(u64, f64)>) -> Vec<(u64, u64)> {
        let mut results = Vec::new();

        let pt = &self.learning_results.patterns;
        let apt = &self.learning_results.anti_patterns;
        
        for i in 0..ts.len() {

            let mut pattern_match_score = 0f64;
            let mut pattern_match_len = 0usize;
            let mut anti_pattern_match_score = 0f64;

            for p in pt {
                if i + p.len() < ts.len() {
                    let mut backet = Vec::<f64>::new();
                    for j in 0..p.len() {
                        backet.push(nan_to_zero(ts[i + j].1));
                    }
                    let score = PatternDetector::corr_aligned(p, &backet);
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
                    let score = PatternDetector::corr_aligned(p, &backet);
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
                let fs = PatternDetector::get_features(&backet);
                let detected = self.learning_results.model.lock().predict(Array::from_vec(fs.to_vec()));
                if detected {
                    pattern_match_score += 0.1;
                } else {
                    anti_pattern_match_score += 0.1;
                }
            }

            if pattern_match_score > anti_pattern_match_score && pattern_match_score >= SCORE_THRESHOLD {
                results.push((ts[i].0, ts[i + pattern_match_len - 1].0));
            }
        }

        return results;
    }

    fn corr_aligned(xs: &Vec<f64>, ys: &Vec<f64>) -> f64 {
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

        // TODO: case when denominator = 0

        let result: f64 = numerator / denominator;

        // assert!(result.abs() <= 1.01);

        if result.abs() > 1.1 {
            println!("{:?}", xs);
            println!("------------");
            println!("{:?}", ys);
            println!("WARNING: corr result > 1: {}", result);
        }

        return result;
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

        return [
            min,
            max, 
            mean,
            sd,
            0f64,0f64,
            //0f64,0f64,0f64, 0f64
        ];
    }

}
