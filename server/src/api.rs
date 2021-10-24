use hastic::services::user_service;
use warp::http::HeaderValue;
use warp::hyper::Body;
use warp::{Rejection, Reply, body};
use warp::{Filter, http::Response };
use warp::filters::method::post;


mod auth;

use serde::{ Serialize };


pub struct API {


}





impl API {
    fn new() -> API {
        API{}
    }

    fn builder<T>(s: T) -> Result<Response<T>, warp::http::Error> {
        return Response::builder()
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "POST, GET, OPTIONS, DELETE")
            .header("Access-Control-Allow-Headers", "*")
            .body(s)
    }

    fn json<T: Serialize>(t: &T) -> Response<Body> {
        let j = warp::reply::json(t);
        let mut rs = j.into_response();
        let hs = rs.headers_mut();
        hs.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
        hs.insert("Access-Control-Allow-Methods", HeaderValue::from_static("POST, GET, OPTIONS, DELETE"));
        hs.insert("Access-Control-Allow-Headers", HeaderValue::from_static("*"));
        rs
    }

    pub async fn serve() {
        let lg = warp::any().map(move || API::builder("not found") );
        let login = auth::get_route();

        println!("Start server on 8000 port");
        warp::serve(login.
            or(lg)
            
        )
            .run(([127, 0, 0, 1], 8000))
            .await;
    }
}