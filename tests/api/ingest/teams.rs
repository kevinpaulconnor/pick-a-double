use crate::helpers::spawn_app;

#[tokio::test]
async fn ingest_teams_returns_a_200_for_valid_api_response() {
    let app = spawn_app().await;
    let response = app.ingest_teams().await;
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn ingest_teams_returns_a_500_when_required_api_response_data_is_missing() {
    // // Arrange
    // let app = spawn_app().await;
    // let test_cases = vec![
    //     ("", "missing email and first_name and last_name"),
    //     (
    //         "email=ursula_le_guin%40gmail.com",
    //         "missing first_name and last_name",
    //     ),
    //     ("first_name=Ursula", "missing email and last_name"),
    //     ("last_name=Le_Guin", "missing email and first_name"),
    //     (
    //         "email=ursula_le_guin%40gmail.com&first_name=Ursula",
    //         "missing last_name",
    //     ),
    //     (
    //         "email=ursula_le_guin%40gmail.com&last_name=Le_Guin",
    //         "missing first_name",
    //     ),
    //     ("first_name=Ursula&last_name=Le_Guin", "missing email"),
    // ];

    // for (invalid_body, error_message) in test_cases {
    //     // Act
    //     let response = app.post_users(invalid_body.into()).await;

    //     // Assert
    //     assert_eq!(
    //         400,
    //         response.status().as_u16(),
    //         "The API did not return a 400 Bad Request when the payload was {}.",
    //         error_message
    //     );
    // }
}

#[tokio::test]
async fn ingest_teams_returns_a_500_when_required_api_response_data_is_present_but_invalid() {
    // // Arrange
    // let app = spawn_app().await;
    // let test_cases = vec![
    //     (
    //         "email=&first_name=&last_name=",
    //         "empty email, first_name, and last_name",
    //     ),
    //     (
    //         "email=ursula_le_guin%40gmail.com&first_name=&last_name=",
    //         "empty first_name and last_name",
    //     ),
    //     (
    //         "email=&first_name=Ursula&last_name=",
    //         "empty email and last_name",
    //     ),
    //     (
    //         "email=&first_name=&last_name=Le_Guin",
    //         "empty email and first_name",
    //     ),
    //     ("email=&first_name=Ursula=&last_name=Le_Guin", "empty email"),
    // ];
    // for (body, description) in test_cases {
    //     // Act
    //     let response = app.post_users(body.into()).await;

    //     // Assert
    //     assert_eq!(
    //         400,
    //         response.status().as_u16(),
    //         "The API did not return a 400 Bad Request when the payload was {}.",
    //         description
    //     );
    // }
}
