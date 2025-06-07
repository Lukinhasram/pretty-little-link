#[tokio::main]
async fn main() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

    // Tracing configuration
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "link_shortener=debug, tower_http=debug, axum::rejection=trace".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("tracing running!");
}
