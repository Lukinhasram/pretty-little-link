mod common;
use axum_test::TestServer;
use http::StatusCode;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let test_app = common::spawn_app().await;
    let server = TestServer::new(test_app.app.clone()).unwrap();

    // Act
    let response = server.get("/health").await;

    // Assert
    assert_eq!(response.status_code(), StatusCode::OK);
}
