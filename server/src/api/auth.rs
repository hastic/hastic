use hastic::services::user_service;

use warp::http::HeaderValue;
use warp::{Rejection, Reply};
use warp::filters::method::post;
use warp::{Filter, http::Response };

use serde::{ Serialize };

#[derive(Serialize)]
struct SigninResp {
    token: user_service::AccessToken
}

pub fn get_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    return warp::path!("api" / "auth" / "signin")
        .and(post())
        .and(warp::body::json())
        .map(|user: user_service::User| {
            let token = "asdsad".to_string();
            // user_service::
            let j = warp::reply::json(&SigninResp{ token });
            let mut rs = j.into_response();
            let hs = rs.headers_mut();
            hs.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
            hs.insert("Access-Control-Allow-Methods", HeaderValue::from_static("POST, GET, OPTIONS, DELETE"));
            hs.insert("Access-Control-Allow-Headers", HeaderValue::from_static("*"));

            rs
            // API::builder(j)
        });
}