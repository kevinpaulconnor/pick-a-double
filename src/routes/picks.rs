use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    game_id: i32,
    player_id: i32,
}

pub async fn select_player_game(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
