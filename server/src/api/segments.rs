pub mod filters {
    use super::handlers;
    use super::models::{Db, ListOptions};
    use warp::Filter;

    /// The 4 REST API filters combined.
    pub fn filters(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        list(db.clone())
            .or(create(db.clone()))
            // .or(update(db.clone()))
            .or(delete(db.clone()))
    }

    /// GET /segments?from=3&to=5
    pub fn list(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("segments")
            .and(warp::get())
            .and(warp::query::<ListOptions>())
            .and(with_db(db))
            .and_then(handlers::list)
    }

    /// POST /segments with JSON body
    pub fn create(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("segments")
            .and(warp::post())
            .and(warp::body::json())
            .and(with_db(db))
            .and_then(handlers::create)
    }

    /// POST /segments with JSON body
    pub fn delete(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("segments")
            .and(warp::delete())
            .and(warp::query::<ListOptions>())
            .and(with_db(db))
            .and_then(handlers::delete)
    }

    fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || db.clone())
    }
}

mod handlers {
    use hastic::services::segments_service;
    use hastic::services::segments_service::Segment;

    use super::models::{CreateResponse, Db, ListOptions};
    use crate::api;
    use crate::api::BadQuery;
    use crate::api::API;

    pub async fn list(opts: ListOptions, db: Db) -> Result<impl warp::Reply, warp::Rejection> {
        match db.read().get_segments_intersected(opts.from, opts.to) {
            Ok(segments) => Ok(API::json(&segments)),
            Err(e) => Err(warp::reject::custom(BadQuery)),
        }
    }

    pub async fn create(
        segment: segments_service::Segment,
        db: Db,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        match db.write().insert_segment(&segment) {
            Ok(segment) => Ok(API::json(&segment)),
            Err(e) => {
                println!("{:?}", e);
                Err(warp::reject::custom(BadQuery))
            }
        }
    }

    pub async fn delete(opts: ListOptions, db: Db) -> Result<impl warp::Reply, warp::Rejection> {
        match db.read().delete_segments_in_range(opts.from, opts.to) {
            Ok(count) => Ok(API::json(&api::Message {
                message: count.to_string(),
            })),
            Err(e) => Err(warp::reject::custom(BadQuery)),
        }
    }
}

mod models {
    use hastic::services::segments_service::{self, SegmentId};
    use parking_lot::RwLock;
    use serde::{Deserialize, Serialize};
    use std::sync::Arc;

    pub type Db = Arc<RwLock<segments_service::SegmentsService>>;

    // The query parameters for list_todos.
    #[derive(Debug, Deserialize)]
    pub struct ListOptions {
        pub from: u64,
        pub to: u64,
    }

    #[derive(Debug, Serialize)]
    pub struct CreateResponse {
        pub id: SegmentId,
    }
}
