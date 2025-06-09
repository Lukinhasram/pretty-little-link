pub mod errors;
pub mod models;
pub mod routes;
pub mod services;

use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
}