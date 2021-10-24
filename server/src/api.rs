use warp::{Rejection, Reply, body};
use warp::{Filter, http::Response };
use warp::filters::method::post;


mod auth;


pub struct API {


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
        let login = auth::get_route();

        println!("Start server on 8000 port");
        warp::serve(login.
            or(lg)
        )
            .run(([127, 0, 0, 1], 8000))
            .await;
    }
}