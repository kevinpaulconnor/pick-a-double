use crate::helpers::spawn_app;

#[tokio::test]
async fn create_user_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let body = format!(
        "email={}&first_name={}&last_name={}",
        format!("ursula_le_guin%40gmail.com"),
        "Ursula",
        "Le_Guin"
    );
    let response = client
        .post(&format!("{}/create_user", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn insert_user_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("", "missing email and first_name and last_name"),
        (
            "email=ursula_le_guin%40gmail.com",
            "missing first_name and last_name",
        ),
        ("first_name=Ursula", "missing email and last_name"),
        ("last_name=Le_Guin", "missing email and first_name"),
        (
            "email=ursula_le_guin%40gmail.com&first_name=Ursula",
            "missing last_name",
        ),
        (
            "email=ursula_le_guin%40gmail.com&last_name=Le_Guin",
            "missing first_name",
        ),
        ("first_name=Ursula&last_name=Le_Guin", "missing email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/select_player_game", &app.address))
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
