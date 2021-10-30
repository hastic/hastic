pub mod filters {
    use super::handlers;
    use super::models::{Srv, ListOptions};
    use warp::Filter;

    /// The 4 REST API filters combined.
    pub fn filters(
        srv: Srv,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        list(srv.clone())
            // .or(create(db.clone()))
            // // .or(update(db.clone()))
            // .or(delete(db.clone()))
    }
        
    /// GET /analytics?from=3&to=5
    pub fn list(
        db: Srv,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("analytics")
            .and(warp::get())
            .and(warp::query::<ListOptions>())
            .and(with_srv(db))
            .and_then(handlers::list)
    }

    fn with_srv(srv: Srv) -> impl Filter<Extract = (Srv,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || srv.clone())
    }
}

mod handlers {

    use super::models::{ListOptions, Srv};
    use crate::api::{API, BadQuery};
    
    pub async fn list(opts: ListOptions, srv: Srv) -> Result<impl warp::Reply, warp::Rejection> {
        match srv.get_detections(opts.from, opts.to, 10).await {
            Ok(segments) => Ok(API::json(&segments)),
            Err(e) => Err(warp::reject::custom(BadQuery)),
        }
    }
}

mod models {
    use hastic::services::analytic_service;
    use serde::{Deserialize, Serialize};

    // use parking_lot::RwLock;
    // use std::sync::Arc;

    pub type Srv = analytic_service::AnalyticService;

    // The query parameters for list_todos.
    #[derive(Debug, Deserialize)]
    pub struct ListOptions {
        pub from: u64,
        pub to: u64,
    }
}