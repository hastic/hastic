use warp::{Filter, http::Response };
use warp::filters::method::post;

use crate::user_service;


pub struct API {


}


fn json_body() -> impl Filter<Extract = (user_service::User,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

impl API {
    fn new() -> API {
        API{}
    }

    fn builder(s: &str) -> Result<Response<String>, warp::http::Error> {
        return Response::builder()
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "POST, GET, OPTIONS, DELETE")
            .header("Access-Control-Allow-Headers", "*")
            .body(s.to_owned())
    }

    pub async fn serve() {
        let lg = warp::any().map(move || API::builder("not found") );
        let login = warp::any()
            // path!("api" / "auth" / "signin")
            .and(post())
            .and(json_body())
            .map(|user: user_service::User| {
                let s = format!("Hello, {}!", &user.username);
                API::builder(&s)
            });

        println!("Start server on 8000 port");
        warp::serve(login.or(lg))
            .run(([127, 0, 0, 1], 8000))
            .await;
    }
}