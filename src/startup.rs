use crate::routes::{health_check, select_player_game};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/select_player_game", web::post().to(select_player_game))
    })
    .listen(listener)?
    .run();
    Ok(server)
}