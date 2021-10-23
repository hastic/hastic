use warp::{Filter, http::Response };
use warp::filters::method::post;


mod user_service;
mod api;



#[tokio::main]
async fn main() {
    api::API::serve().await;
}