use super::super::game_server;
use super::websockets;
use actix::Addr;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::time::Instant;

pub async fn play(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<game_server::GameServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        websockets::ws_game_session::WsGameSession {
            id: "".to_owned(),
            heart_beat: Instant::now(),
            room: "Main".to_owned(),
            game_server_addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}
