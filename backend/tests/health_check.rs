mod common;
use axum_test::TestServer;
use http::StatusCode;

#[tokio::test]
async fn health_check_works() {
    let test_app = common::spawn_app().await;
    let server = TestServer::new(test_app.app.clone()).unwrap();

    let response = server.get("/health").await;

    assert_eq!(response.status_code(), StatusCode::OK);
}
