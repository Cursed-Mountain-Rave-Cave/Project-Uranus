use actix_web::{middleware, App, HttpServer};

mod http_handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var(
        "RUST_LOG",
        "debug,my_errors=debug,actix_server=info,actix_web=info",
    );
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(http_handlers::health_check::handle_request)
    })
    .bind("localhost:4000")?
    .run()
    .await
}
