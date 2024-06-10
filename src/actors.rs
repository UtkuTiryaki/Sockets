use crate::messages::*;
use crate::session::WebSocketSession;
use actix::prelude::*;
use std::collections::HashMap;

pub struct DashboardActor {
    sessions: HashMap<String, Addr<WebSocketSession>>,
}

impl DashboardActor {
    pub fn new() -> Self {
        DashboardActor {
            sessions: HashMap::new()
        }
    }
}

impl Actor for DashboardActor {
    type Context = Context<Self>;
}

impl Handler<Connect> for DashboardActor {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        self.sessions.insert(msg.connection_id.clone(), msg.addr);
        println!("User {} connected", msg.connection_id);
    }
}

impl Handler<Disconnect> for DashboardActor {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        self.sessions.remove(&msg.connection_id);
        println!("User {} disconnected", msg.connection_id);
    }
}