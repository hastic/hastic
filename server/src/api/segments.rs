use hastic::services::user_service;

use warp::filters::method::post;
use warp::http::HeaderValue;
use warp::hyper::{Body, StatusCode};
use warp::{http::Response, Filter};
use warp::{Rejection, Reply};

use serde::Serialize;

use crate::api::{self, API};

use parking_lot::RwLock;
use std::sync::Arc;


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
    let from = p.get("from").unwrap().parse::<u64>()?;
    let to = p.get("to").unwrap().parse::<u64>()?;

    let prom = ms.read().get_prom();
    drop(ms);
    // Ok(prom.query(from, to, step).await?)
}

pub fn get_route(
    user_service: Arc<RwLock<user_service::UserService>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    return warp::path!("api" / "segments")
        .and(get())
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::any().map(move || metric_service.clone()))
        .and_then(query);
}
