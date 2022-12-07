#[tokio::test]
async fn health_check_works() {
    spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(r#"http://127.0.0.1:8888/health_check"#)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background
fn spawn_app() {
    let server = rustchimp::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}
