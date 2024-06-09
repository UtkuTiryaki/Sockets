use actix::{Addr, Message};
use serde::{Deserialize, Serialize};

#[derive(Message, Deserialize, Serialize, Debug, Clone)]
#[rtype(result = "()")]
pub struct JoinGroup {
    pub group_id: String,
    pub connection_id: String,
}

#[derive(Message, Deserialize, Serialize, Debug, Clone)]
#[rtype(result = "()")]
pub struct LeaveGroup {
    pub group_id: String,
    pub connection_id: String,
}

#[derive(Message, Deserialize, Serialize, Debug, Clone)]
#[rtype(result = "()")]
pub struct GroupMessage {
    pub group_id: String,
    pub message: String,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Addr<crate::session::WebSocketSession>,
    pub connection_id: String,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub connection_id: String,
}

#[derive(Message, Debug, Clone)]
#[rtype(result = "()")]
pub struct SendTextMessage {
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type")]
pub enum ClientMessage {
    JoinGroup {
        group_id: String,
        connection_id: String,
    },
    GroupMessage {
        group_id: String,
        message: String,
    },
}
#[derive(Deserialize, Serialize, Debug)]
pub struct ServerMessage {
    pub message: String,
}