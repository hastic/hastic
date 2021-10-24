use hastic::services::user_service;
use warp::filters::method::post;
use warp::http::HeaderValue;
use warp::hyper::{Body, StatusCode};
use warp::{body, options, Rejection, Reply};
use warp::{http::Response, Filter};

mod auth;

use serde::Serialize;

use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Serialize)]
struct Options {
    message: String,
}

pub struct API {
    user_service: Arc<RwLock<user_service::UserService>>,
}

impl API {
    pub fn new() -> API {
        API {
            user_service: Arc::new(RwLock::new(user_service::UserService::new())),
        }
    }

    fn json<T: Serialize>(t: &T) -> Response<Body> {
        let j = warp::reply::json(t);
        let mut rs = j.into_response();
        let hs = rs.headers_mut();
        hs.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
        hs.insert(
            "Access-Control-Allow-Methods",
            HeaderValue::from_static("POST, GET, OPTIONS, DELETE"),
        );
        hs.insert(
            "Access-Control-Allow-Headers",
            HeaderValue::from_static("*"),
        );
        rs
    }

    pub async fn serve(&self) {
        let not_found =
            warp::any().map(|| warp::reply::with_status("Not found", StatusCode::NOT_FOUND));
        let options = warp::any().and(options()).map(|| {
            API::json(&Options {
                message: "ok".to_owned(),
            })
        });
        let login = auth::get_route(self.user_service.clone());

        println!("Start server on 8000 port");
        let routes = login.or(options).or(not_found);
        warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
    }
}
