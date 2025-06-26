mod health;
mod links;

use crate::AppState;
use axum::{
    Router,
    routing::{get, post},
};
use http::{Method};
use tower_http::cors::{Any, CorsLayer};

pub fn app_router(app_state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    Router::new()
        .route("/health", get(health::health_check_handler))
        .route("/shorten", post(links::create_short_link_handler))
        .route("/{short_code}", get(links::redirect_handler))
        .with_state(app_state)
        .layer(cors)
}
