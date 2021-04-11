use actix::*;
use actix_web_actors::ws;

use crate::game_server;
use crate::game_server::requests::player_turn;
use crate::utils;

use std::time::{Duration, Instant};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WsGameSession {
    pub id: String,
    pub heart_beat: Instant,
    pub room: String,
    pub game_server_addr: Addr<game_server::GameServer>,
}

impl Actor for WsGameSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, context: &mut Self::Context) {
        self.heart_beat(context);

        let addr = context.address();
        self.game_server_addr
            .send(game_server::requests::connect::Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, context| {
                match res {
                    Ok(res) => act.id = res,
                    _ => context.stop(),
                }
                fut::ready(())
            })
            .wait(context);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // TODO - send server disconnect message
        Running::Stop
    }
}

impl Handler<game_server::requests::Message> for WsGameSession {
    type Result = ();

    fn handle(&mut self, msg: game_server::requests::Message, context: &mut Self::Context) {
        context.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsGameSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, context: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                context.stop();
                return;
            }
            Ok(msg) => msg,
        };

        println!("WEBSOCKET MESSAGE: {:?}", msg);
        match msg {
            ws::Message::Ping(msg) => {
                self.heart_beat = Instant::now();
                context.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.heart_beat = Instant::now();
            }
            ws::Message::Text(text) => {
                println!("Received text: {}\n", text);
                let player_turn: player_turn::PlayerTurn = utils::json::decode(text.trim());
                self.game_server_addr
                    .send(player_turn)
                    .into_actor(self)
                    .then(|res, _, context| {
                        match res {
                            Ok(player_turn_response) => {
                                println!("Send answer: {:?}", player_turn_response);
                                context.text(utils::json::encode(&player_turn_response))
                            },
                            _ => context.stop(),
                        }
                        fut::ready(())
                    })
                    .wait(context);
            }
            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Close(reason) => {
                context.close(reason);
                context.stop();
            }
            ws::Message::Continuation(_) => {
                context.stop();
            }
            ws::Message::Nop => (),
        }
    }
}

impl WsGameSession {
    fn heart_beat(&self, context: &mut ws::WebsocketContext<Self>) {
        context.run_interval(HEARTBEAT_INTERVAL, |act, context| {
            if Instant::now().duration_since(act.heart_beat) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // TODO - send server disconnect message
                // act.addr.do_send();

                context.stop();
            }

            context.ping(b"");
        });
    }
}
