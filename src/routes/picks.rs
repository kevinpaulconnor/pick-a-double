use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    user_id: String,
    game_id: String,
    player_id: String,
}

#[tracing::instrument(name = "Adding pick to picks table", skip(form, pool))]
pub async fn insert_pick(form: &FormData, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO picks (id, user_id, game_id, player_id, created_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        Uuid::new_v4(),
        Uuid::parse_str(&form.user_id).expect("Failed to parse UUID."),
        Uuid::parse_str(&form.game_id).expect("Failed to parse UUID."),
        Uuid::parse_str(&form.player_id).expect("Failed to parse UUID."),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query. {:?}", e);
        e
    })?;
    Ok(())
}

#[tracing::instrument(
    name = "Adding a new pick",
    skip(_form, _pool),
    fields(
        player_id = %_form.player_id,
        user_id = %_form.user_id,
        game_id = %_form.game_id
    )
)]
pub async fn select_player_game(
    _form: web::Form<FormData>,
    _pool: web::Data<PgPool>,
) -> HttpResponse {
    match insert_pick(&_form, &_pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
