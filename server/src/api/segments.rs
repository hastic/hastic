pub mod filters {
    use super::handlers;
    use super::models::{ListOptions, Srv};
    use hastic::services::analytic_service::analytic_client::AnalyticClient;
    use warp::Filter;

    /// The 4 REST API filters combined.
    pub fn filters(
        srv: Srv,
        ac: AnalyticClient,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        list(srv.clone())
            .or(create(srv.clone(), ac.clone()))
            // .or(update(db.clone()))
            .or(delete(srv.clone(), ac.clone()))
    }

    /// GET /segments?from=3&to=5
    pub fn list(
        srv: Srv,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("segments")
            .and(warp::get())
            .and(warp::query::<ListOptions>())
            .and(with_srv(srv))
            .and_then(handlers::list)
    }

    /// POST /segments with JSON body
    pub fn create(
        srv: Srv,
        ac: AnalyticClient,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("segments")
            .and(warp::post())
            .and(warp::body::json())
            .and(with_srv(srv))
            .and(warp::any().map(move || ac.clone()))
            .and_then(handlers::create)
    }

    /// POST /segments with JSON body
    pub fn delete(
        srv: Srv,
        ac: AnalyticClient,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("segments")
            .and(warp::delete())
            .and(warp::query::<ListOptions>())
            .and(with_srv(srv))
            .and(warp::any().map(move || ac.clone()))
            .and_then(handlers::delete)
    }

    fn with_srv(
        srv: Srv,
    ) -> impl Filter<Extract = (Srv,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || srv.clone())
    }
}

mod handlers {
    use hastic::services::analytic_service::analytic_client::AnalyticClient;
    use hastic::services::segments_service;

    use super::models::{ListOptions, Srv};
    use crate::api;
    use crate::api::BadQuery;
    use crate::api::API;

    pub async fn list(opts: ListOptions, src: Srv) -> Result<impl warp::Reply, warp::Rejection> {
        match src.get_segments_intersected(opts.from, opts.to) {
            Ok(segments) => Ok(API::json(&segments)),
            // TODO: return proper http error
            Err(_e) => Err(warp::reject::custom(BadQuery)),
        }
    }

    pub async fn create(
        segment: segments_service::Segment,
        srv: Srv,
        ac: AnalyticClient,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        match srv.insert_segment(&segment) {
            Ok(segment) => {
                ac.run_learning().await.unwrap();
                Ok(API::json(&segment))
            }
            Err(e) => {
                println!("{:?}", e);
                // TODO: return proper http error
                Err(warp::reject::custom(BadQuery))
            }
        }
    }

    pub async fn delete(
        opts: ListOptions,
        srv: Srv,
        ac: AnalyticClient,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        match srv.delete_segments_in_range(opts.from, opts.to) {
            Ok(count) => {
                ac.run_learning().await.unwrap();
                Ok(API::json(&api::Message {
                    message: count.to_string(),
                }))
            }
            // TODO: return proper http error
            Err(_e) => Err(warp::reject::custom(BadQuery)),
        }
    }
}

mod models {
    use hastic::services::segments_service::{self, SegmentId};
    use serde::{Deserialize, Serialize};

    pub type Srv = segments_service::SegmentsService;

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
