use actix::{Addr, Message};
use serde::{Deserialize, Serialize};

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
pub struct ClientMessage {
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ServerMessage {
    pub message: String,
}