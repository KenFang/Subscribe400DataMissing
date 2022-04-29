use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange HttpServer::run
    let address = spawn_app();

    // We need to bring in 'reqwest'
    // to perform HTTP requests against our application
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background
// tokio::spawn runs concurrently with down stream futures and tasks; our test logic.
fn spawn_app() -> String {
    let address = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    // We retrieve the port assigned to us by the OS
    let port = address.local_addr().unwrap().port();
    let server = zero2prod::run(address).expect("Failed to bind address");

    // Launch the server as a background task
    // tokio::spam returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);

    // We return the application address to the caller!
    format!("http://127.0.0.1:{}", port)
}