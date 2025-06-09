use axum::Json;
use serde_json::{json, Value};

pub async fn health_check_handler() -> Json<Value> {
    tracing::info!("Health check endpoint was called.");
    Json(json!({
        "status": "OK"
    }))
}