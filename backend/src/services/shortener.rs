use crate::errors::AppError;
use rand::{Rng, distributions::Alphanumeric};
use sqlx::PgPool;
use std::env;

const SHORT_CODE_LENGTH: usize = 7;

fn generate_short_code() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(SHORT_CODE_LENGTH)
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
                let base_url = env::var("BACKEND_URL")
                    .unwrap_or_else(|_| "http://localhost:3000".to_string());

                // Construct the full URL using the environment-aware base_url
                let full_url = format!("{}/{}", base_url, short_code);
                return Ok(full_url);
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_short_code_length() {
        let code = generate_short_code();
        assert_eq!(code.len(), SHORT_CODE_LENGTH);
    }

    #[test]
    fn test_generate_short_code_uniqueness_sanity_check() {
        let code1 = generate_short_code();
        let code2 = generate_short_code();
        assert_ne!(code1, code2);
    }
}
