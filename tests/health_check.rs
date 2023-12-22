use std::net::TcpListener;

use paperboy::run;

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn test_health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/health_check", address))
        .send()
        .await
        .expect("Request Failed");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

/*
 * Abstract out the framework specific implementation detail.
 * All our tests need to know is that there is a running process that accepts HTTP connections
 * This prevents us needing to rewrite test suite if we decide to change the web framework
 * */
fn spawn_app() -> String {
    // Port 0 is a special OS level feature
    // it triggers an OS scan of avaiable ports to bind to and binds to one
    // This allows use to run multiple tests in parallel by binding to different ports
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // Retrive the port that has been assigned to us by the OS for the test requests
    // This is also why we create our own listener instead of having HttpServer do that work internally
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");

    // Launch server in a background task
    // tokio::spawn returns a handle to the spawned future
    let _ = tokio::spawn(server);

    // Return addr to the caller
    format!("http://127.0.0.1:{}", port)
}
