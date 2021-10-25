use subbeat::datasources::prometheus::Prometheus;

pub struct MetricService {
    url: String,
    query: String,
}

impl MetricService {
    pub fn new(url: &str, query: &str) -> MetricService {
        MetricService {
            url: url.to_string(),
            query: url.to_string(),
        }
    }

    pub fn get_prom(&self) -> Prometheus {
        Prometheus::new(&self.url.to_string(), &self.query.to_string())
    }
}
