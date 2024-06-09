use crate::messages::*;
use crate::session::WebSocketSession;
use actix::prelude::*;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

pub struct DashboardActor {
    sessions: HashMap<String, Addr<WebSocketSession>>,
    groups: Arc<Mutex<HashMap<String, HashSet<String>>>>,
}

impl DashboardActor {
    pub fn new() -> Self {
        DashboardActor {
            sessions: HashMap::new(),
            groups: Arc::new(Mutex::new(HashMap::new())),
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

        let mut groups = self.groups.lock().unwrap();
        for group in groups.values_mut() {
            if group.remove(&msg.connection_id) {
                println!("User {} removed from group", msg.connection_id);
            }
        }
    }
}

impl Handler<JoinGroup> for DashboardActor {
    type Result = ();

    fn handle(&mut self, msg: JoinGroup, _: &mut Self::Context) -> Self::Result {
        let mut groups = self.groups.lock().unwrap();
        groups
            .entry(msg.group_id.clone())
            .or_insert_with(HashSet::new)
            .insert(msg.connection_id.clone());
        println!("User {} joined group {}", msg.connection_id, msg.group_id);
    }
}

impl Handler<LeaveGroup> for DashboardActor {
    type Result = ();

    fn handle(&mut self, msg: LeaveGroup, _: &mut Self::Context) -> Self::Result {
        let mut groups = self.groups.lock().unwrap();
        if let Some(group) = groups.get_mut(&msg.group_id) {
            if group.remove(&msg.connection_id) {
                println!("User {} left group {}", msg.connection_id, msg.group_id);
            }
        }
    }
}

impl Handler<GroupMessage> for DashboardActor {
    type Result = ();

    fn handle(&mut self, msg: GroupMessage, _: &mut Self::Context) -> Self::Result {
        let groups = self.groups.lock().unwrap();
        if let Some(group) = groups.get(&msg.group_id) {
            println!("Sending message to group {}: {}", msg.group_id, msg.message);
            for connection_id in group {
                if let Some(addr) = self.sessions.get(connection_id) {
                    addr.do_send(SendTextMessage {
                        message: msg.message.clone(),
                    });
                }
            }
        }
    }
}