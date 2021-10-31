
struct LearningResults {
    backet_size: usize
}

pub struct PatternDetector {
    learning_results: Option<LearningResults>
}

impl PatternDetector {
    pub fn new() -> PatternDetector {
        PatternDetector{
            learning_results: None
        }
    }

    pub fn learn(&mut self, reads: &Vec<Vec<(u64, f64)>>) {
        // TODO: implement
        let mut min_size = usize::MAX;
        let mut max_size = 0usize;
        for r in reads {
            min_size = min_size.min(r.len());
            max_size = max_size.max(r.len());
        }

        self.learning_results = Some(LearningResults{
            backet_size: (min_size + max_size) / 2
        });
    }

    pub fn detect(&self, ts: &Vec<(u64, f64)>) -> Vec<(u64, u64)> {
        // fill backet
        return Vec::new();
    }
}