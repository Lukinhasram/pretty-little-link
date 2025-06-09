use axum::Json;
use serde_json::{Value, json};

pub async fn health_check_handler() -> Json<Value> {
    tracing::info!("Health check endpoint was called.");
    Json(json!({
        "status": "OK"
    }))
}
