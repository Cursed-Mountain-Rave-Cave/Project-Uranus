mod handlers;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(handlers::hello)
            .service(handlers::echo)
            .route("/hey", web::get().to(handlers::manual_hello))
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await
}
