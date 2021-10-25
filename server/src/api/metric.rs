use hastic::services::{metric_service, user_service};

use warp::filters::method::get;
use warp::http::HeaderValue;
use warp::hyper::{Body, StatusCode};
use warp::{http::Response, Filter};
use warp::{Rejection, Reply};

use serde::Serialize;

use crate::api::{self, API};

use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Serialize)]
struct QueryResponse {}

pub fn get_route(
    metric_service: Arc<RwLock<user_service::UserService>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    return warp::path!("api" / "metric")
        .and(get())
        .and(warp::query::<HashMap<String, String>>())
        .map(move |p: HashMap<String, String>| {
            let qr = QueryResponse {};
            return api::API::json(&qr);
        });
}
