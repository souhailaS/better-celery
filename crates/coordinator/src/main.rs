mod api;
mod db;

use axum::{Router, routing::{get, post}};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{fmt, EnvFilter};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,tower_http=info".into());
    fmt().with_env_filter(filter).init();

    let pool = db::make_pool().await?;
    let state = api::AppState { pool };

    let app = Router::new()
        .route("/health", get(api::health))
        .route("/v1/jobs", post(api::create_job))
        .route("/v1/jobs/:id", get(api::get_job))
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("coordinator listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}