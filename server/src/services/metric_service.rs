use subbeat::{datasources::prometheus::Prometheus, metric::{Metric, MetricResult}};

// TODO: use resolve function as in subbeat itself

pub struct MetricService {
    // url: String,
    // query: String,

    datasource: Box<dyn Metric + Sync + Send>
}

impl Clone for MetricService {
    fn clone(&self) -> Self {
        return MetricService {
            datasource: self.datasource.boxed_clone()
        }
    }
}

impl MetricService {
    pub fn new(ds_config: &subbeat::types::DatasourceConfig) -> MetricService {
        MetricService {
            // url: url.to_string(),
            // query: query.to_string(),
            datasource: subbeat::datasources::resolve(ds_config)
        }
    }

    pub async fn query(&self, from: u64, to: u64, step: u64) -> anyhow::Result<MetricResult> {
        return self.datasource.query(from, to, step).await;
    }

}

