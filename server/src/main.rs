mod api;

use hastic::services::{analytic_service, metric_service, segments_service};

use anyhow;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = hastic::config::Config::new();

    let metric_service = metric_service::MetricService::new(&config.prom_url, &config.query);
    let segments_service = segments_service::SegmentsService::new()?;

    let mut analytic_service =
        analytic_service::AnalyticService::new(metric_service.clone(), segments_service.clone());

    let api = api::API::new(
        &config,
        metric_service.clone(),
        segments_service.clone(),
        analytic_service.get_client(),
    );

    let s1 = analytic_service.serve();
    let s2 = api.serve();

    futures::future::join(s1, s2).await;

    Ok(())
}
