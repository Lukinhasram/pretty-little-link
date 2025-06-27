use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use url::Url;

#[derive(Deserialize)]
pub struct CreateLinkRequest {
    pub original_url: Url,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShortLinkResponse {
    pub short_url: String,
}

#[derive(Debug, FromRow)]
pub struct Link {
    pub id: i32,
    pub short_code: String,
    pub original_url: String,
}
