use subbeat::{
    metric::{Metric, MetricResult},
};

pub struct MetricService {
    datasource: Box<dyn Metric + Sync + Send>,
}

impl Clone for MetricService {
    fn clone(&self) -> Self {
        return MetricService {
            datasource: self.datasource.boxed_clone(),
        };
    }
}

impl MetricService {
    pub fn new(ds_config: &subbeat::types::DatasourceConfig) -> MetricService {
        MetricService {
            datasource: subbeat::datasources::resolve(ds_config),
        }
    }
    pub async fn query(&self, from: u64, to: u64, step: u64) -> anyhow::Result<MetricResult> {
        return self.datasource.query(from, to, step).await;
    }
}
