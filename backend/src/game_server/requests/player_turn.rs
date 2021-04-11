use crate::game_server::requests::Message;
use crate::game_server::GameServer;
use actix::dev::{MessageResponse, ResponseChannel};
use actix::prelude::*;
use serde::{ Serialize, Deserialize };

#[derive(Message, Debug, Deserialize)]
#[rtype(result = "PlayerTurnResponse")]
pub struct PlayerTurn {
    pub session_id: String,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Serialize)]
pub struct PlayerTurnResponse {
    your_turn: bool,
    game_status: String,
    last_taken_cell: (i32, i32),
}

impl<A, M> MessageResponse<A, M> for PlayerTurnResponse
where
    A: Actor,
    M: actix::prelude::Message<Result = PlayerTurnResponse>,
{
    fn handle<R: ResponseChannel<M>>(self, _: &mut A::Context, tx: Option<R>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}

impl Handler<PlayerTurn> for GameServer {
    type Result = PlayerTurnResponse;

    fn handle(&mut self, message: PlayerTurn, _: &mut Context<Self>) -> Self::Result {
        PlayerTurnResponse {
            your_turn: false,
            game_status: "".to_owned(),
            last_taken_cell: (0, 0),
        }
    }
}
