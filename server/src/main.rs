mod api;

#[tokio::main]
async fn main() {
    api::API::serve().await;
}
