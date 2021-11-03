use subbeat::{datasources::prometheus::Prometheus, metric::{Metric, MetricResult}};

// TODO: use resolve function as in subbeat itself
#[derive(Clone)]
pub struct MetricService {
    // url: String,
    // query: String,

    prom: Prometheus
}

impl MetricService {
    pub fn new(url: &str, query: &str) -> MetricService {
        MetricService {
            // url: url.to_string(),
            // query: query.to_string(),
            prom: Prometheus::new(&url.to_string(), &query.to_string())
        }
    }

    pub async fn query(&self, from: u64, to: u64, step: u64) -> anyhow::Result<MetricResult> {
        return self.prom.query(from, to, step).await;
    }

}

