use subbeat::datasources::prometheus::Prometheus;

#[derive(Clone)]
pub struct MetricService {
    url: String,
    query: String,
}

impl MetricService {
    pub fn new(url: &str, query: &str) -> MetricService {
        MetricService {
            url: url.to_string(),
            query: query.to_string(),
        }
    }

    // TODO: make prom as field, but Prometheus should be clonable first
    pub fn get_prom(&self) -> Prometheus {
        Prometheus::new(&self.url.to_string(), &self.query.to_string())
    }
}
