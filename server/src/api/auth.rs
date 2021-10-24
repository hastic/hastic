use crate::api::API;
use crate::services::user_service;

use warp::{Rejection, Reply};
use warp::filters::method::post;
use warp::{Filter, http::Response };


pub fn get_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    return warp::path!("api" / "auth" / "signin")
        .and(post())
        .and(warp::body::json())
        .map(|user: user_service::User| {
            let s = format!("Hello, {}!", &user.username);
            API::builder(&s)
        });
}