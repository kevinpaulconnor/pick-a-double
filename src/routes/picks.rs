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

pub async fn select_player_game(
    _form: web::Form<FormData>,
    _pool: web::Data<PgPool>,
) -> HttpResponse {
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
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => {
            println!("Failed to execute query.");
            HttpResponse::InternalServerError().finish()
        }
    }
}
