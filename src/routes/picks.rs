use actix_web::{web, HttpResponse};

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct FormData {
    user_id: i32,
    game_id: i32,
    player_id: i32,
}

pub async fn select_player_game(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
