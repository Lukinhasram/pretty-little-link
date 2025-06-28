mod health;
mod links;

use crate::AppState;
use axum::{
    Router,
    routing::{get, post},
};
use http::{Method};
use std::env;
use tower_http::cors::{Any, CorsLayer};

pub fn app_router(app_state: AppState) -> Router {
    let cors = match env::var("FRONTEND_URL") {
        // If the FRONTEND_URL is set (in production)
        Ok(frontend_url) => {
            tracing::info!("CORS configured for production origin: {}", &frontend_url);
            CorsLayer::new()
                // ...allow requests only from that specific origin.
                .allow_origin(frontend_url.parse::<http::HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
                .allow_headers(Any)
        }
        // If the FRONTEND_URL is NOT set (in local development)
        Err(_) => {
            tracing::info!("CORS configured for development (permissive)");
            CorsLayer::new()
                // ...allow requests from any origin.
                .allow_origin(tower_http::cors::Any)
                .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
                .allow_headers(Any)
        }
    };

    Router::new()
        .route("/health", get(health::health_check_handler))
        .route("/shorten", post(links::create_short_link_handler))
        .route("/{short_code}", get(links::redirect_handler))
        .with_state(app_state)
        .layer(cors)
}
