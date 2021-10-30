use anyhow;

mod api;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = hastic::config::Config::new();
    let api = api::API::new(&config)?;

    api.serve().await;

    Ok(())
}
