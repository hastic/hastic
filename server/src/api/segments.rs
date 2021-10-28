use hastic::services::data_service::{self, Segment};

use warp::filters::method::get;
use warp::http::HeaderValue;
use warp::hyper::{Body, StatusCode};
use warp::reject::Reject;
use warp::{http::Response, Filter};
use warp::{Rejection, Reply};

use serde::Serialize;

use crate::api::{self, API};

use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Serialize)]
struct SegmentsResult {
    segments: Vec<Segment>
}

#[derive(Debug)]
struct BadQuery;

impl Reject for BadQuery {}



async fn get_query(
    p: HashMap<String, String>,
    ds: Arc<RwLock<data_service::DataService>>,
) -> anyhow::Result<SegmentsResult> {
    if !p.contains_key("from") {
        return Err(anyhow::anyhow!("Missing attribute from"));
    }
    if !p.contains_key("to") {
        return Err(anyhow::anyhow!("Missing attribute to"));
    }
    let from = p.get("from").unwrap().parse::<u64>()?;
    let to = p.get("to").unwrap().parse::<u64>()?;

    let res = ds.read().get_segments(from, to)?;
    drop(ds);

    Ok(SegmentsResult{ segments: res })
    // Ok(prom.query(from, to, step).await?)
}

async fn query(
    p: HashMap<String, String>,
    ds: Arc<RwLock<data_service::DataService>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    //Err(warp::reject::custom(BadQuery));
    match get_query(p, ds).await {
        Ok(res) => Ok(API::json(&res)),
        // TODO: parse different error types
        Err(_e) => Err(warp::reject::custom(BadQuery)),
    }
}

pub fn get_route(
    data_service: Arc<RwLock<data_service::DataService>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    
    return warp::path!("api" /  "segments")
        .and(get())
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::any().map(move || data_service.clone()))
        .and_then(query);
}
