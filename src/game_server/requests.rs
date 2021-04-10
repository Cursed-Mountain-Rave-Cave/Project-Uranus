use actix::prelude::*;
//use crate::game_server::GameServer;
pub mod connect;

/// Chat server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);
