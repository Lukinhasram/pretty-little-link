mod health;
mod links;

use axum::{Router, routing::{get, post}};
use crate::AppState;

pub fn app_router(app_state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health_check_handler))
        .route("/shorten", post(links::create_short_link_handler))
        .route("/:short_code", get(links::redirect_handler)).with_state(app_state)
}
