use dotenvy::dotenv;
use link_shortener::{AppState, routes};
use std::net::SocketAddr;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Tracing setup
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            "link_shortener=debug, tower_http=debug, axum::rejection=trace".into()
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();
    tracing::info!("Tracing running.");

    // Create AppState
    let app_state = AppState { db_pool };

    // Router setup
    let app = routes::app_router(app_state);

    // Running the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
