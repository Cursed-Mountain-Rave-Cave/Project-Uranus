mod http_handlers;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(http_handlers::hello)
            .service(http_handlers::echo)
            .service(http_handlers::play)
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await
}
