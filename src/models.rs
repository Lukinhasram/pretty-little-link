use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct CreateLinkRequest {
    pub original_url: String,
}

#[derive(Serialize)]
pub struct ShortLinkResponse {
    pub short_url: String,
}

#[derive(Debug, FromRow)]
pub struct Link {
    pub id: i32,
    pub short_code: String,
    pub original_url: String,
}
