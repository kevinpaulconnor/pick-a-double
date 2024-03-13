use pick_a_double::startup::run;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("http://{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("127.0.0.1:{}", port)
}

#[tokio::test]
async fn select_player_game_returns_a_200_for_valid_form_data() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let body = "email=ursula_le_guin%40gmail.com&game_id=1&player_id=1";

    // Act
    let response = client
        .post(&format!("http://{}/select_player_game", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn select_player_game_returns_a_400_when_data_is_missing() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("", "missing email and game_id and player_id"),
        (
            "email=ursula_le_guin%40gmail.com",
            "missing game_id and player_id",
        ),
        ("game_id=1", "missing email and player_id"),
        ("player_id=1", "missing email and game_id"),
        (
            "email=ursula_le_guin%40gmail.com&game_id=1",
            "missing player_id",
        ),
        (
            "email=ursula_le_guin%40gmail.com&player_id=1",
            "missing game_id",
        ),
        ("game_id=1&player_id=1", "missing email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("http://{}/select_player_game", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
