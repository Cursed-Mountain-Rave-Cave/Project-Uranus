use actix_web::{get, HttpResponse, Responder};

#[get("/health_check")]
pub async fn handle_request() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
