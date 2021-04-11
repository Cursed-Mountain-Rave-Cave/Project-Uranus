use crate::game_server::requests::*;
use actix_web::{
    get, post,
    web::{Bytes, Data},
    HttpResponse, Responder,
};

mod requests;
mod responses;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/play")]
pub async fn play(
    bytes: Bytes,
    game_server: Data<actix::Addr<crate::game_server::GameServer>>,
) -> impl Responder {
    log::debug!("game_server: {:?}", game_server);

    let body: requests::GameRequest = requests::decode(
        &String::from_utf8(bytes.to_vec())
            .map_err(|_| HttpResponse::BadRequest().finish())
            .unwrap(),
    );
    log::debug!("player_id: {}", body.player_id);

    let result = game_server
        .send(register::Register {
            player_id: body.player_id.clone(),
        })
        .await;

    let response = match result {
        Ok(session_id) => responses::Play {
            code: "ok".to_owned(),
            session_id,
        },
        _ => responses::Play {
            code: "error".to_owned(),
            session_id: "".to_owned(),
        },
    };

    HttpResponse::Ok().body(responses::encode(&response))
}
