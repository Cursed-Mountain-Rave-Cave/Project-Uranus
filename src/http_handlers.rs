use actix_web::{get, post, web, HttpResponse, Responder};
use uuid::Uuid;
mod responses;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/play")]
pub async fn play(game_server: web::Data<actix::Addr<crate::game_server::GameServer>>) -> impl Responder {
    println!("game_server: {:?}", game_server);
    let session_id = Uuid::new_v4().to_hyphenated().to_string();
    let response = responses::Play{session_id};
    HttpResponse::Ok().body(responses::encode(&response))
}