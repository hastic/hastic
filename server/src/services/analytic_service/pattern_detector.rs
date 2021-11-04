#[derive(Debug, Clone)]
pub struct LearningResults {
    // model: Vec<f64>,
    patterns: Vec<Vec<f64>>,
}

const CORR_THRESHOLD: f64 = 0.95;

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

    pub async fn learn(reads: &Vec<Vec<(u64, f64)>>) -> LearningResults {
        // let size_avg = reads.iter().map(|r| r.len()).sum::<usize>() / reads.len();

        // let mut stat = Vec::<(usize, f64)>::new();
        // for _i in 0..size_avg {
        //     stat.push((0usize, 0f64));
        // }

        let mut patterns = Vec::<Vec<f64>>::new();

        // for r in reads {
        //     let xs: Vec<f64> = r.iter().map(|e| e.1).map(nan_to_zero).collect();
        //     if xs.len() > size_avg {
        //         let offset = (xs.len() - size_avg) / 2;
        //         for i in 0..size_avg {
        //             stat[i].0 += 1;
        //             stat[i].1 += xs[i + offset];
        //         }
        //     } else {
        //         let offset = (size_avg - xs.len()) / 2;
        //         for i in 0..xs.len() {
        //             stat[i + offset].0 += 1;
        //             stat[i + offset].1 += xs[i];
        //         }
        //     }
        // }

        for r in reads {
            let xs: Vec<f64> = r.iter().map(|e| e.1).map(nan_to_zero).collect();
            patterns.push(xs);
        }

        // let model = stat.iter().map(|(c, v)| v / *c as f64).collect();

        LearningResults {
            patterns
            //model
        }
    }

    // TODO: get iterator instead of vector
    pub fn detect(&self, ts: &Vec<(u64, f64)>) -> Vec<(u64, u64)> {
        let mut results = Vec::new();
        // let mut i = 0;

        // let m = &self.learning_results.model;

        // // TODO: here we ignoring gaps in data
        // while i < ts.len() - self.learning_results.model.len() {
        //     let mut backet = Vec::<f64>::new();

        //     for j in 0..m.len() {
        //         backet.push(nan_to_zero(ts[j + i].1));
        //     }

        //     let c = PatternDetector::corr_aligned(&backet, &m);

        //     if c >= CORR_THRESHOLD {
        //         let from = ts[i].0;
        //         let to = ts[i + backet.len() - 1].0;
        //         results.push((from, to));
        //     }

        //     i += m.len();
        // }

        let pt = &self.learning_results.patterns;

        for i in 0..ts.len() {
            for p in pt {
                if i + p.len() < ts.len() {
                    let mut backet = Vec::<f64>::new();
                    for j in 0..p.len() {
                        backet.push(nan_to_zero(ts[i + j].1));
                    }
                    if PatternDetector::corr_aligned(p, &backet) >= CORR_THRESHOLD {
                        results.push((ts[i].0, ts[i + p.len() - 1].0));
                    }
                }
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

        // TODO: case when denominator = 0

        let result: f64 = numerator / denominator;

        // assert!(result.abs() <= 1.01);

        if result.abs() > 1.0 {
            println!("WARNING: corr result > 1: {}", result);
        }

        return result;
    }
}
