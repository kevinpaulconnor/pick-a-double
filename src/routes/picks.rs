use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    user_id: String,
    game_id: String,
    player_id: String,
}

pub async fn select_player_game(
    _form: web::Form<FormData>,
    _pool: web::Data<PgPool>,
) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new pick",
        %request_id,
        player_id = %_form.player_id,
        user_id = %_form.user_id,
        game_id = %_form.game_id
    );
    let _request_span_guard = request_span.enter();
    let query_span = tracing::info_span!("Adding pick to picks table", %request_id);
    match sqlx::query!(
        r#"
        INSERT INTO picks (id, user_id, game_id, player_id, created_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        Uuid::new_v4(),
        Uuid::parse_str(&_form.user_id).expect("Failed to parse UUID."),
        Uuid::parse_str(&_form.game_id).expect("Failed to parse UUID."),
        Uuid::parse_str(&_form.player_id).expect("Failed to parse UUID."),
        Utc::now()
    )
    .execute(_pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!(
                "request_id {} - Successfully added pick {} for user {} in game {}.",
                request_id,
                _form.player_id,
                _form.user_id,
                _form.game_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "request_id {} - Failed to execute query. {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
