use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    game_id: i32,
    player_id: i32,
}

async fn select_player_game(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new().route("/health_check", web::get().to(health_check))
        .route("/select_player_game", web::post().to(select_player_game))
        })
        .listen(listener)?
        .run();
    Ok(server)
}
