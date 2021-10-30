use subbeat::metric::MetricResult;

struct PatternDetector {

}

impl PatternDetector {
    fn new() -> PatternDetector {
        PatternDetector{}
    }

    fn learn(reads: &Vec<Vec<(u64, f64)>>) {
        // TODO: implement
    }

    fn detect(ts: &Vec<(u64, f64)>) -> Vec<(u64, u64)> {
        // fill backet
        return Vec::new();
    }
}