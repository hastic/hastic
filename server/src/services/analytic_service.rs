use crate::config::Config;

use super::{metric_service::MetricService, segments_service::{Segment, SegmentType}};

use subbeat::metric::Metric;

use anyhow;


#[derive(Clone)]
pub struct AnalyticService {
    metric_service: MetricService,
}

impl AnalyticService {
    pub fn new(config: &Config) -> AnalyticService {
        AnalyticService {
            metric_service: MetricService::new(&config.prom_url, &config.query),
        }
    }

    pub async fn get_detections(&self, from: u64, to: u64, step: u64) -> anyhow::Result<Vec<Segment>> {
        let prom = self.metric_service.get_prom();
        let mr = prom.query(from, to, step).await?;

        if mr.data.keys().len() == 0 {
            return Ok(Vec::new());
        }

        let key = mr.data.keys().nth(0).unwrap();
        let ts = &mr.data[key];

        let mut result = Vec::<Segment>::new();
        let mut from: Option<u64> = None;
        for (t, v) in ts {
            if *v > 10_000.0 {
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

        if from.is_some() {
            result.push(Segment {
                id: None,
                from: from.unwrap(),
                to,
                segment_type: SegmentType::Detection,
            });
        }

        // TODO: decide what to do it from is Some() in the end

        Ok(result)
    }
}
