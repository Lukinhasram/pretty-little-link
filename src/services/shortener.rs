use crate::errors::AppError;
use rand::{Rng, distributions::Alphanumeric};
use sqlx::PgPool;
use std::panic::resume_unwind;

const SHORT_CODE_LENGHT: usize = 7;

fn generate_short_code() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(SHORT_CODE_LENGHT)
        .map(char::from)
        .collect()
}

pub async fn create_short_link(db_pool: &PgPool, original_url: &str) -> Result<String, AppError> {
    // Loop that handles code collision
    loop {
        let short_code = generate_short_code();

        // Try to insert the new link into the database
        let result = sqlx::query("INSERT INTO links (short_code, original_url) VALUES ($1, $2)")
            .bind(&short_code)
            .bind(original_url)
            .execute(db_pool)
            .await;

        match result {
            Ok(_) => {
                let full_url = format!("http://localhost:3000/{}", short_code);
            }
            Err(e) => {
                if let Some(db_err) = e.as_database_error() {
                    if db_err.is_unique_violation() {
                        continue;
                    }
                }

                return Err(AppError::InternalServerError(
                    "Failed to create short link".to_string(),
                ));
            }
        }
    }
}

pub async fn find_long_url(db_pool: &PgPool, short_code: &str) -> Result<String, AppError> {
    let result = sqlx::query_as::<_, crate::models::Link>(
        "SELECT id, short_code, original_url FROM links WHERE short_code = $1",
    )
    .bind(short_code)
    .fetch_one(db_pool)
    .await;

    match result {
        Ok(link) => Ok(link.original_url),

        Err(sqlx::Error::RowNotFound) => Err(AppError::NotFound("Link not found.".to_string())),

        Err(_) => Err(AppError::InternalServerError(
            "Failed to query database.".to_string(),
        )),
    }
}
