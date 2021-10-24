use hastic::services::user_service;

use warp::filters::method::post;
use warp::http::HeaderValue;
use warp::hyper::Body;
use warp::{http::Response, Filter};
use warp::{Rejection, Reply};

use serde::Serialize;

use crate::api;

use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Serialize)]
struct SigninResp {
    token: user_service::AccessToken,
}

pub fn get_route(user_service: Arc<RwLock<user_service::UserService>>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    return warp::path!("api" / "auth" / "signin")
        .and(post())
        .and(warp::body::json())
        .map(move |user: user_service::User| {
            let us = user_service.write().login(&user);
            match us {
                Some(token) => api::API::json(&SigninResp { token }),
                None => api::API::json(&SigninResp { token: "no token".to_string() })
            }
            
        });
}
