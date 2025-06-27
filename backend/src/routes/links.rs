use crate::{
    AppState,
    errors::AppError,
    models::{CreateLinkRequest, ShortLinkResponse},
    services::shortener,
};

use axum::{
    Json,
    extract::{Path, State},
    response::Redirect,
    http::StatusCode,
};

use tracing::info;

pub async fn create_short_link_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateLinkRequest>,
) -> Result<(StatusCode, Json<ShortLinkResponse>), AppError> {
    info!("Create short link endpoint was called.");

    let short_url = shortener::create_short_link(&state.db_pool, &payload.original_url.as_str()).await?;
    let response = ShortLinkResponse { short_url };

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn redirect_handler(
    State(state): State<AppState>,
    Path(short_code): Path<String>,
) -> Result<Redirect, AppError> {
    info!("Redirect endpoint was called.");

    let original_url = shortener::find_long_url(&state.db_pool, &short_code).await?;

    Ok(Redirect::to(&original_url))
}
