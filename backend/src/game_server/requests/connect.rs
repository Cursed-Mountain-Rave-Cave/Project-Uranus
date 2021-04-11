use crate::game_server::requests::Message;
use crate::game_server::GameServer;
use actix::prelude::*;

/// New chat session is created
#[derive(Message, Debug)]
#[rtype(String)]
pub struct Connect {
    pub addr: Recipient<Message>,
}

impl Handler<Connect> for GameServer {
    type Result = String;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!("Someone joined!");
        println!("Msg: {:?}", msg);
        println!("Server state: {:?}", self);

        // register session with random id
        let id = "".to_owned();
        //self.sessions.insert(id, msg.addr);

        //// auto join session to Main room
        //self.rooms
        //    .entry("Main".to_owned())
        //    .or_insert_with(HashSet::new)
        //    .insert(id);

        //let count = self.visitor_count.fetch_add(1, Ordering::SeqCst);
        //self.send_message("Main", &format!("Total visitors {}", count), 0);

        //// send id back
        id
    }
}
