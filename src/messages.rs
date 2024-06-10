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

#[derive(Deserialize, Serialize, Debug, Message, Clone)]
#[rtype(result = "()")]
#[serde(tag = "type")]
pub enum ClientMessage {
    MousePosition { x: f64, y: f64 },
}

#[derive(Deserialize, Serialize, Debug, Message, Clone)]
#[rtype(result = "()")]
#[serde(tag = "type")]
pub enum ServerMessage {
    TransformedMousePosition { x: f64, y: f64 },
}