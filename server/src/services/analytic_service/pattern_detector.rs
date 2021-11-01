use std::{thread, time};
use tokio::time::{sleep, Duration};

#[derive(Debug, Clone)]
pub struct LearningResults {
    backet_size: usize,
    // avg_min: f64,
    // avg_max: f64,
}

#[derive(Clone)]
pub struct PatternDetector {
    learning_results: LearningResults,
}

// TODO: move this to loginc of analytic unit
impl PatternDetector {
    pub fn new(learning_results: LearningResults) -> PatternDetector {
        PatternDetector { learning_results }
    }

    pub async fn learn(reads: &Vec<Vec<(u64, f64)>>) -> LearningResults {
        // TODO: implement
        let mut min_size = usize::MAX;
        let mut max_size = 0usize;
        for r in reads {
            min_size = min_size.min(r.len());
            max_size = max_size.max(r.len());
        }

        // let mut max_sum = 0;
        // let mut min_sum = 0;

        // for read in reads {
        //     let my_max = read.iter().map(|(t,v)| *v).max().unwrap();
        //     let my_min = read.iter().min().unwrap();
        // }

        LearningResults {
            backet_size: (min_size + max_size) / 2,
        }
    }

    // TODO: get iterator instead of vector
    pub fn detect(&self, ts: &Vec<(u64, f64)>) -> Vec<(u64, u64)> {
        let mut results = Vec::new();
        let mut i = 0;
        while i < ts.len() - self.learning_results.backet_size {
            let backet: Vec<_> = ts
                .iter()
                .skip(i)
                .take(self.learning_results.backet_size)
                .collect();

            let mut min = f64::MAX;
            let mut max = f64::MIN;

            for (t, v) in backet.iter() {
                min = v.min(min);
                max = v.max(max);
            }

            if min > 10_000. && max < 100_000. {
                let from = backet[0].0;
                let to = backet[backet.len() - 1].0;
                results.push((from, to));
            }

            i += 100;
        }

        return results;
    }
}
