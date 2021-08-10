mod api_server;
mod game_server;

use actix::Actor;
use actix_web::{middleware, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug,my_errors=debug,actix_server=info,actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let server = game_server::GameServer::new().start();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(server.clone())
            .service(api_server::http_handlers::hello)
            .service(api_server::http_handlers::echo)
            .service(api_server::http_handlers::play)
            .service(actix_web::web::resource("/ws/").to(api_server::ws_handlers::play))
    })
    .bind("localhost:4000")?
    .run()
    .await
}
