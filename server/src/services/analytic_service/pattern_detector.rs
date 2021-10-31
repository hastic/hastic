use std::{thread, time};
use tokio::time::{sleep, Duration};

#[derive(Clone)]
pub struct LearningResults {
    backet_size: usize,
}

#[derive(Clone)]
pub struct PatternDetector {
    learning_results: LearningResults,
}

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

        let ten_millis = time::Duration::from_millis(1000);
        thread::sleep(ten_millis);

        sleep(Duration::from_millis(1000)).await;

        LearningResults {
            backet_size: (min_size + max_size) / 2,
        }
    }

    pub fn detect(&self, ts: &Vec<(u64, f64)>) -> Vec<(u64, u64)> {
        // fill backet
        return Vec::new();
    }
}
