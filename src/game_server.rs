use actix::prelude::*;
use std::collections::HashMap;
use rand::{self, rngs::ThreadRng, /*Rng*/};
mod room;
use room::Room;
pub mod requests;
use requests::Message;

#[derive(Debug)]
pub struct GameServer {
    sessions: HashMap<usize, Recipient<Message>>,
    rooms: HashMap<usize, Room>,
    rng: ThreadRng, 
}

impl GameServer {
    pub fn new() -> GameServer {
        GameServer {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
            rng: rand::thread_rng(),
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

