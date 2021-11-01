pub mod filters {
    use super::handlers;
    use super::models::{Client, ListOptions};
    use warp::Filter;

    /// The 4 REST API filters combined.
    pub fn filters(
        client: Client,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        list(client.clone()).or(status(client.clone()))
        // .or(create(db.clone()))
        // // .or(update(db.clone()))
        // .or(delete(db.clone()))
    }

    /// GET /analytics?from=3&to=5
    pub fn list(
        client: Client,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("analytics")
            .and(warp::get())
            .and(warp::query::<ListOptions>())
            .and(with_client(client))
            .and_then(handlers::list)
    }

    /// GET /analytics/status
    pub fn status(
        client: Client,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("analytics" / "status")
            .and(warp::get())
            .and(with_client(client))
            .and_then(handlers::status)
    }

    fn with_client(
        client: Client,
    ) -> impl Filter<Extract = (Client,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || client.clone())
    }
}

mod handlers {

    use super::models::{Client, ListOptions, Status};
    use crate::api::{BadQuery, API};

    pub async fn list(
        opts: ListOptions,
        client: Client,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        // match srv.get_threshold_detections(opts.from, opts.to, 10, 100_000.).await {
        match client.get_pattern_detection(opts.from, opts.to).await {
            Ok(segments) => Ok(API::json(&segments)),
            Err(e) => {
                println!("{:?}", e);
                Err(warp::reject::custom(BadQuery))
            }
        }
    }

    pub async fn status(client: Client) -> Result<impl warp::Reply, warp::Rejection> {
        match client.get_status().await {
            Ok(ls) => Ok(API::json(&Status { status: ls })),
            Err(e) => {
                println!("{:?}", e);
                Err(warp::reject::custom(BadQuery))
            }
        }
    }
}

mod models {
    use hastic::services::analytic_service::{self, types::LearningStatus};
    use serde::{Deserialize, Serialize};

    pub type Client = analytic_service::analytic_client::AnalyticClient;

    // The query parameters for list_todos.
    #[derive(Debug, Deserialize)]
    pub struct ListOptions {
        pub from: u64,
        pub to: u64,
    }

    #[derive(Debug, Serialize)]
    pub struct Status {
        pub status: LearningStatus,
    }
}