use crate::helpers::spawn_app;
use uuid::Uuid;

#[tokio::test]
async fn select_player_game_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    // this probably isn't the right way to do this
    let user_id = Uuid::new_v4();
    let game_id = Uuid::new_v4();
    let player_id = Uuid::new_v4();
    let body = format!(
        "user_id={}&game_id={}&player_id={}",
        user_id, game_id, player_id
    );

    // Act
    let response = client
        .post(&format!("{}/select_player_game", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!("SELECT user_id, game_id, player_id FROM picks")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved player_game.");

    assert!(saved.user_id == user_id);
    assert!(saved.game_id == game_id);
    assert!(saved.player_id == player_id);
}

#[tokio::test]
async fn select_player_game_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("", "missing player_id and game_id and user_id"),
        (
            "player_id=00000000-0000-0000-0000-000000000000",
            "missing game_id and user_id",
        ),
        (
            "game_id=00000000-0000-0000-0000-000000000000",
            "missing player_id and user_id",
        ),
        (
            "user_id=00000000-0000-0000-0000-000000000000",
            "missing player_id and game_id",
        ),
        (
            "player_id=00000000-0000-0000-0000-000000000000&game_id=00000000-0000-0000-0000-000000000000",
            "missing user_id",
        ),
        (
            "player_id=00000000-0000-0000-0000-000000000000&user_id=00000000-0000-0000-0000-000000000000",
            "missing game_id",
        ),
        (
            "game_id=00000000-0000-0000-0000-000000000000&user_id=00000000-0000-0000-0000-000000000000",
            "missing player_id",
        ),
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
