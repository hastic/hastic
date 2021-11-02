use hastic::config::Config;
use hastic::services::analytic_service::{analytic_client, AnalyticService};
use hastic::services::{metric_service, segments_service, user_service};
use warp::http::HeaderValue;
use warp::hyper::{Body, StatusCode};
use warp::reject::Reject;
use warp::{http::Response, Filter};
use warp::{options, Reply};

mod analytics;
mod auth;
mod metric;
mod segments;

use serde::Serialize;

use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Debug)]
struct BadQuery;

impl Reject for BadQuery {}

#[derive(Serialize)]
pub struct Message {
    message: String,
}

pub struct API<'a> {
    config: &'a Config,
    user_service: Arc<RwLock<user_service::UserService>>,
    metric_service: metric_service::MetricService,
    segments_service: segments_service::SegmentsService,
    analytic_client: analytic_client::AnalyticClient,
}

impl API<'_> {
    pub fn new(
        config: &Config,
        metric_service: metric_service::MetricService,
        segments_service: segments_service::SegmentsService,
        analytic_client: analytic_client::AnalyticClient,
    ) -> API<'_> {
        API {
            config: config,
            user_service: Arc::new(RwLock::new(user_service::UserService::new())),
            metric_service,
            segments_service,
            analytic_client,
        }
    }

    fn json<T: Serialize>(t: &T) -> Response<Body> {
        API::json_with_code(t, StatusCode::OK)
    }

    fn json_with_code<T: Serialize>(t: &T, status_code: StatusCode) -> Response<Body> {
        let j = warp::reply::json(t);
        let mut rs = j.into_response();
        let hs = rs.headers_mut();
        hs.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
        hs.insert(
            "Access-Control-Allow-Methods",
            HeaderValue::from_static("POST, GET, OPTIONS, DELETE"),
        );
        hs.insert(
            "Access-Control-Allow-Headers",
            HeaderValue::from_static("*"),
        );
        *rs.status_mut() = status_code;
        rs
    }

    pub async fn serve(&self) {
        let not_found =
            warp::any().map(|| warp::reply::with_status("Not found", StatusCode::NOT_FOUND));
        let options = warp::any().and(options()).map(|| {
            API::json(&Message {
                message: "ok".to_owned(),
            })
        });
        let metrics = metric::get_route(self.metric_service.clone());
        let login = auth::get_route(self.user_service.clone());
        let segments =
            segments::filters::filters(self.segments_service.clone(), self.analytic_client.clone());
        let analytics = analytics::filters::filters(self.analytic_client.clone());
        let public = warp::fs::dir("public");

        println!("Start server on {} port", self.config.port);
        // TODO: move it to "server"
        let routes = warp::path("api")
            .and(login.or(metrics).or(segments).or(analytics).or(options))
            .or(public)
            .or(not_found);

        warp::serve(routes)
            .run(([127, 0, 0, 1], self.config.port))
            .await;
    }
}
