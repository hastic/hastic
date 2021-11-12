use subbeat::metric::{Metric, MetricResult};

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
        let mut mr = self.datasource.query(from, to, step).await?;
        // let keys: Vec<_> = mr.data.keys().into_iter().collect();

        if mr.data.keys().len() > 0 {
        // TODO: it's a hack, should replace all metrics
            let key = mr.data.keys().nth(0).unwrap().clone();
            let ts = mr.data.get_mut(&key).unwrap();
            *ts = subbeat::utils::interpolate_nans_and_gaps_with_zeros(&ts, from, to, step);
                // mr.data.insert(*k, ts_interpolated);
        }
        return Ok(mr);
    }
}
