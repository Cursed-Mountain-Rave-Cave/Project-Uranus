use crate::game_server::requests::Message;
use crate::game_server::GameServer;
use actix::prelude::*;

/// New chat session is created
#[derive(Message, Debug)]
#[rtype(String)]
pub struct Register {
    pub player_id: String,
}

impl Handler<Register> for GameServer {
    type Result = String;

    fn handle(&mut self, msg: Register, _: &mut Context<Self>) -> Self::Result {
        println!("Someone joined!");
        println!("Msg: {:?}", msg);
        println!("Server state: {:?}", self);

        let id = uuid::Uuid::new_v4().to_hyphenated().to_string();
        self.players_to_sessions.insert(msg.player_id, id.clone());

        id
    }
}
