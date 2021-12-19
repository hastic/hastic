mod api;

use hastic::services::{analytic_service, metric_service, segments_service, analytic_unit_service};

use anyhow;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = hastic::config::Config::new()?;
    let cfg_clone = config.clone();

    let analytic_unit_service = analytic_unit_service::AnalyticUnitService::new()?;
    let metric_service = metric_service::MetricService::new(&config.datasource_config);
    let segments_service = segments_service::SegmentsService::new()?;

    let mut analytic_service = analytic_service::AnalyticService::new(
        analytic_unit_service.clone(),
        metric_service.clone(),
        segments_service.clone(),
        config.alerting,
    );

    let api = api::API::new(
        &cfg_clone,
        metric_service.clone(),
        segments_service.clone(),
        analytic_service.get_client(),
    );

    let s1 = analytic_service.serve();
    let s2 = api.serve();

    futures::future::join(s1, s2).await;

    Ok(())
}
