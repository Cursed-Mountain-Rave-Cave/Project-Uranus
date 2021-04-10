use actix::prelude::*;
use std::collections::HashMap;

mod room;
use room::Room;
pub mod requests;
use requests::Message;

#[derive(Debug)]
pub struct GameServer {
    sessions: HashMap<String, Recipient<Message>>,
    rooms: HashMap<String, Room>,
    players_to_sessions: HashMap<String, String>,
}

impl GameServer {
    pub fn new() -> GameServer {
        GameServer {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
            players_to_sessions: HashMap::new(),
        }
    }
}

/// Make actor from `ChatServer`
impl Actor for GameServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Started GameServer actor!");
        println!("Context: {:?}", ctx);
    }
}
