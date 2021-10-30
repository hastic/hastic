use crate::config::Config;

use super::{metric_service::MetricService, segments_service::{Segment, SegmentType}};

use subbeat::metric::Metric;

use anyhow;



struct AnalyticService {
    metric_service: MetricService,
}

impl AnalyticService {
    fn new(config: &Config) -> AnalyticService {
        AnalyticService {
            metric_service: MetricService::new(&config.prom_url, &config.query),
        }
    }

    pub async fn get_detections(&self, from: u64, to: u64) -> anyhow::Result<Vec<Segment>> {
        let prom = self.metric_service.get_prom();
        let mr = prom.query(from, to, 10).await?;

        let key = mr.data.keys().nth(0).unwrap();
        let ts = &mr.data[key];

        let mut result = Vec::<Segment>::new();
        let mut from: Option<u64> = None;
        for (t, v) in ts {
            if *v > 100.0 {
                if from.is_some() {
                    continue;
                } else {
                    from = Some(*t);
                }
            } else {
                if from.is_some() {
                    result.push(Segment {
                        id: None,
                        from: from.unwrap(),
                        to: *t,
                        segment_type: SegmentType::Detection,
                    });
                    from = None;
                }
            }
        }

        Ok(result)
    }
}
