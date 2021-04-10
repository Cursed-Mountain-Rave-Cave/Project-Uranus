mod http_handlers;
mod game_server;
use actix_web::{App, HttpServer};
use actix::Actor;

use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web::{middleware, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// do websocket handshake and start `MyWebSocket` actor
async fn ws_index(r: HttpRequest, stream: web::Payload, game_server: web::Data<Addr<game_server::GameServer>>) -> Result<HttpResponse, Error> {
    println!("{:?}", r);
    let res = ws::start(MyWebSocket::new(game_server.get_ref().clone()), &r, stream);
    println!("{:?}", res);
    res
}

// websocket connection is long running connection, it easier
/// to handle with an actor
struct MyWebSocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    id: usize,
    hb: Instant,
    game_server: Addr<game_server::GameServer>,
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        // process websocket messages
        println!("WS: {:?}", msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                let addr = ctx.address();
                self.game_server.send(game_server::requests::connect::Connect{
                    addr: addr.recipient()})
                    .into_actor(self)
                    .then(|res, act, ctx| {
                        match res {
                            Ok(res) => {
                                act.id = res;
                                println!("Got id: {:?}", act.id);
                            },
                            _ => ctx.stop(),
                        }
                        fut::ready(())
                    })
                    .wait(ctx);
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

/// Handle messages from chat server, we simply send it to peer websocket
impl Handler<game_server::requests::Message> for MyWebSocket{
    type Result = ();

    fn handle(&mut self, msg: game_server::requests::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl MyWebSocket {
    fn new(game_server: Addr<game_server::GameServer>) -> Self {
        Self { hb: Instant::now(), game_server, id: 0 }
    }

    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    let server = game_server::GameServer::new().start();     

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(server.clone())
            .service(web::resource("/ws/").route(web::get().to(ws_index)))
            .service(http_handlers::hello)
            .service(http_handlers::echo)
            .service(http_handlers::play)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

