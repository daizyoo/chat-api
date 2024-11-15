use actix_web::{App, HttpServer};
use tracing_subscriber::EnvFilter;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    HttpServer::new(|| App::new())
        .bind(("127.0.0.1", 3478))?
        .run()
        .await?;

    Ok(())
}
