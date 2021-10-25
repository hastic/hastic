use subbeat::datasources::prometheus::Prometheus;

pub struct MetricService {
    prom: Prometheus,
}

impl MetricService {
    pub fn new(url: &str, query: &str) -> MetricService {
        MetricService {
            prom: Prometheus::new(&url.to_string(), &query.to_string()),
        }
    }
}
