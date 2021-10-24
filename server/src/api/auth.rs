use hastic::services::user_service;

use warp::http::HeaderValue;
use warp::hyper::Body;
use warp::{Rejection, Reply};
use warp::filters::method::post;
use warp::{Filter, http::Response };

use serde::{ Serialize };

use crate::api;

#[derive(Serialize)]
struct SigninResp {
    token: user_service::AccessToken
}



pub fn get_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    return warp::path!("api" / "auth" / "signin")
        .and(post())
        .and(warp::body::json())
        .map(|user: user_service::User| {
            api::API::json(&SigninResp { token: "asdad".to_string() })
        });
}