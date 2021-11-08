pub mod pattern_analytic_unit;
pub mod threshold_analytic_unit;
pub mod types;

use async_trait::async_trait;

#[async_trait]
trait AnalyticUnit<C> {
    async fn learn(reads: &Vec<Vec<(u64, f64)>>, anti_reads: &Vec<Vec<(u64, f64)>>);
    fn detect(&self, ts: &Vec<(u64, f64)>) -> Vec<(u64, u64)>;
}

