mod errors;

use axum::Json;
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
    use axum::{routing::get, Router};

    // Tracing setup
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "link_shortener=debug, tower_http=debug, axum::rejection=trace".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();
    tracing::info!("Tracing running.");

    // Router setup
    let app = Router::new()
        .route("/health", get(health_check_handler));
    tracing::info!("Router configured.");

    // Running the server
    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn health_check_handler() -> Json<Value> {
    tracing::info!("Health check endpoint was called.");
    Json(json!({
        "status": "OK"
    }))
}