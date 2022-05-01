use std::net::TcpListener;

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

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_from_data() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let body = "name=ken%20fang&email=kenfang%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send request");

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=ken%20fang", "missing the email"),
        ("email=kenfang%40gmail.com", "missing the name" ),
        ("", "missing both name and email")
    ];

    // Act & Assert
    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to send request.");

        // Assert
        assert_eq!(
            400, response.status().as_u16(),

            // Customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.", error_message
        );
    }
}
