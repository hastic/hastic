#[derive(Debug, Clone)]
pub struct LearningResults {
    model: Vec<f64>, // avg_min: f64,
                     // avg_max: f64
}

const CORR_THRESHOLD: f64 = 0.9;

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

        let size_avg = reads.iter().map(|r| r.len()).sum::<usize>() / reads.len();

        let mut stat = Vec::<(usize, f64)>::new();
        for _i in 0..size_avg {
            stat.push((0usize, 0f64));
        }

        for r in reads {
            let xs: Vec<f64> = r.iter().map(|e| e.1).map(nan_to_zero).collect();
            if xs.len() > size_avg {
                let offset = (xs.len() - size_avg) / 2;
                for i in 0..size_avg {
                    stat[i].0 += 1;
                    stat[i].1 += xs[i + offset];
                }
            } else {
                let offset = (size_avg - xs.len()) / 2;
                for i in 0..xs.len() {
                    stat[i + offset].0 += 1;
                    stat[i + offset].1 += xs[i];
                }
            }
        }

        let model = stat.iter().map(|(c, v)| v / *c as f64).collect();

        LearningResults { model }
    }

    // TODO: get iterator instead of vector
    pub fn detect(&self, ts: &Vec<(u64, f64)>) -> Vec<(u64, u64)> {
        let mut results = Vec::new();
        let mut i = 0;
        let m = &self.learning_results.model;

        // TODO: here we ignoring gaps in data
        while i < ts.len() - self.learning_results.model.len() {
            let mut backet = Vec::<f64>::new();

            for j in 0..m.len() {
                backet.push(nan_to_zero(ts[j + i].1));
            }

            let c = PatternDetector::corr(&backet, &m);

            if c >= CORR_THRESHOLD {
                let from = ts[i].0;
                let to = ts[i + backet.len() - 1].0;
                results.push((from, to));
            }

            i += m.len();
        }

        return results;
    }

    fn corr(xs: &Vec<f64>, ys: &Vec<f64>) -> f64 {

        assert_eq!(xs.len(), ys.len());

        let n = xs.len() as f64;
        // TODO: compute it faster, with one iteration over x y
        let s_xs: f64 = xs.iter().sum();
        let s_ys: f64 = ys.iter().sum();
        let s_xsys: f64 = xs.iter().zip(ys).map(|(xi, yi)| xi * yi).sum();
        let s_xs_2: f64 = xs.iter().map(|xi| xi * xi).sum();
        let s_ys_2: f64 = ys.iter().map(|yi| yi * yi).sum();

        let numerator: f64 = n * s_xsys - s_xs * s_ys;
        let denominator: f64 = ((n * s_xs_2 - s_xs * s_xs) * (n * s_ys_2 - s_ys * s_ys)).sqrt();

        let result: f64 = numerator / denominator;

        // assert!(result.abs() <= 1.01);

        if result.abs() > 1.0 {
            println!("WARNING: corr result > 1: {}", result);
        }

        return result;
    }

}
