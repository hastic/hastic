use hastic::services::{metric_service, user_service};

use subbeat::metric::Metric;
use warp::filters::method::get;
use warp::http::HeaderValue;
use warp::hyper::server::conn::Http;
use warp::hyper::{Body, StatusCode};
use warp::reject::Reject;
use warp::{http::Response, Filter};
use warp::{reject, Rejection, Reply};

use serde::Serialize;

use crate::api::{self, API};

use parking_lot::RwLock;
use std::collections::HashMap;
use std::convert::Infallible;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Serialize)]
struct QueryResponse {
    message: String,
}

#[derive(Debug)]
struct BadQuery;

impl Reject for BadQuery {}

async fn query(
    p: HashMap<String, String>,
    ms: Arc<RwLock<metric_service::MetricService>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    if !p.contains_key("from") || !p.contains_key("to") || !p.contains_key("step") {
        return Err(warp::reject::custom(BadQuery));
    }
    let from = p.get("from").unwrap().parse::<u64>().unwrap();
    let to = p.get("to").unwrap().parse::<u64>().unwrap();
    let step = p.get("step").unwrap().parse::<u64>().unwrap();

    let prom = ms.read().get_prom();
    drop(ms);
    let res = prom.query(from, to, step).await;
    // let pm = subbeat::datasources::prometheus::Prometheus::new(&"http://".to_owned(), &"asd".to_owned());
    // let r = pm.query(from, to, step).await;

    Ok(API::json(&QueryResponse {
        message: "hello".to_string(),
    }))
}

pub fn get_route(
    metric_service: Arc<RwLock<metric_service::MetricService>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    return warp::path!("api" / "metric")
        .and(get())
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::any().map(move || metric_service.clone()))
        .and_then(query);
}
