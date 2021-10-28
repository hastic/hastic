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

#[derive(Serialize)]
struct SigninResp {
    token: user_service::AccessToken,
}

pub fn get_route(
    user_service: Arc<RwLock<user_service::UserService>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    return warp::path!("auth" / "signin")
        .and(post())
        .and(warp::body::json())
        .map(move |user: user_service::User| {
            let us = user_service.write().login(&user);
            if let Some(token) = us {
                return api::API::json(&SigninResp { token });
            } else {
                return api::API::json_with_code(
                    &api::Message {
                        message: "wrong login or password".to_owned(),
                    },
                    StatusCode::UNAUTHORIZED,
                );
            }
        });
}
