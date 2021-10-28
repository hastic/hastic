use hastic::services::{metric_service, user_service};

use subbeat::metric::{Metric, MetricResult};
use warp::filters::method::get;
use warp::http::HeaderValue;
use warp::hyper::server::conn::Http;
use warp::hyper::{Body, StatusCode};
use warp::reject::Reject;
use warp::{http::Response, Filter};
use warp::{reject, Rejection, Reply};

use serde::Serialize;

use anyhow;

use crate::api::{self, API};

use parking_lot::RwLock;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;
use std::sync::Arc;

use super::BadQuery;

#[derive(Serialize)]
struct QueryResponse {
    message: String,
}


async fn get_query(
    p: HashMap<String, String>,
    ms: Arc<RwLock<metric_service::MetricService>>,
) -> anyhow::Result<MetricResult> {
    if !p.contains_key("from") {
        return Err(anyhow::anyhow!("Missing attribute from"));
    }
    if !p.contains_key("to") {
        return Err(anyhow::anyhow!("Missing attribute to"));
    }
    if !p.contains_key("step") {
        return Err(anyhow::anyhow!("Missing attribute step"));
    }
    let from = p.get("from").unwrap().parse::<u64>()?;
    let to = p.get("to").unwrap().parse::<u64>()?;
    let step = p.get("step").unwrap().parse::<u64>()?;

    let prom = ms.read().get_prom();
    drop(ms);
    Ok(prom.query(from, to, step).await?)
}

async fn query(
    p: HashMap<String, String>,
    ms: Arc<RwLock<metric_service::MetricService>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    //Err(warp::reject::custom(BadQuery));
    match get_query(p, ms).await {
        Ok(res) => Ok(API::json(&res)),
        // TODO: parse different error types
        Err(_e) => Err(warp::reject::custom(BadQuery)),
    }
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
