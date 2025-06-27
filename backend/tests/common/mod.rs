use axum::Router;
use backend::{AppState, routes};
use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::env;
use std::thread;
use tokio::runtime::Runtime;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

// Ensure tracing is only initialized once for all tests
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter = "info".to_string();
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new(default_filter));
    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer().with_test_writer())
        .init();
});

pub struct TestApp {
    pub app: Router,
    db_name: String,
    base_db_url: String,
}

impl Drop for TestApp {
    fn drop(&mut self) {
        let db_name = self.db_name.clone();
        let base_db_url = self.base_db_url.clone();

        let handle = thread::spawn(move || {
            let rt = Runtime::new().expect("Failed to create a Tokio runtime for cleanup");
            rt.block_on(async {
                let mut conn = PgConnection::connect(&format!("{}/postgres", base_db_url))
                    .await
                    .expect("Failed to connect to postgres DB for cleanup");

                // Terminate all other connections to the database.
                let terminate_result = conn
                    .execute(
                        format!(
                            r#"
                        SELECT pg_terminate_backend(pg_stat_activity.pid)
                        FROM pg_stat_activity
                        WHERE pg_stat_activity.datname = '{}' AND pid <> pg_backend_pid();
                        "#,
                            db_name
                        )
                        .as_str(),
                    )
                    .await;

                if let Err(e) = terminate_result {
                    eprintln!("Failed to terminate connections (this might be ok): {}", e);
                }

                // Drop the database. This cannot be in a transaction.
                conn.execute(format!(r#"DROP DATABASE IF EXISTS "{}";"#, db_name).as_str())
                    .await
                    .expect("Failed to drop test database.");
            });
        });

        handle.join().expect("Cleanup thread panicked");
    }
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests");
    let db_name = Uuid::new_v4().to_string();

    let (base_url, _) = db_url
        .rsplit_once('/')
        .expect("Invalid DATABASE_URL format");

    let mut connection = PgConnection::connect(&format!("{}/postgres", base_url))
        .await
        .expect("Failed to connect to Postgres instance");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, db_name).as_str())
        .await
        .expect("Failed to create test database.");

    let test_db_url = format!("{}/{}", base_url, db_name);
    let db_pool = PgPool::connect(&test_db_url)
        .await
        .expect("Failed to connect to the test database");

    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to run migrations");

    let app_state = AppState::new(db_pool.clone());
    let app = routes::app_router(app_state);

    TestApp {
        app,
        db_name,
        base_db_url: base_url.to_string(),
    }
}
