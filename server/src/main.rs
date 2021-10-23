use warp::{Filter, http::Response };
use warp::filters::method::post;


mod user_service;

fn json_body() -> impl Filter<Extract = (user_service::User,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[tokio::main]
async fn main() {
    let routes = warp::any().map(|| { 
        Response::builder()
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "POST, GET, OPTIONS, DELETE")
        .header("Access-Control-Allow-Headers", "*")
        .body("and a custom body")
        // "Hello, World!"
    });
    let hello = warp::path!("api" / "auth" / "signin")
        .and(post())
        .and(json_body())
        .map(|user: user_service::User| {
            format!("Hello, {}!", &user.username)
        });

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8000))
        .await;
}