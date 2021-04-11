use actix::prelude::*;
//use crate::game_server::GameServer;
pub mod connect;
pub mod player_turn;
pub mod register;

/// Chat server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);
