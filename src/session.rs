use actix::prelude::*;
use actix_web_actors::ws;
use serde_json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::actors::DashboardActor;
use crate::messages::{ClientMessage, Connect, Disconnect, ServerMessage};

pub type GroupRegistry = Arc<Mutex<HashMap<String, Vec<Addr<WebSocketSession>>>>>;

pub struct WebSocketSession {
    hb: Instant,
    id: String,
    addr: Addr<DashboardActor>,
    group_id: String,
    registry: GroupRegistry,
    self_addr: Option<Addr<Self>>
}

impl WebSocketSession {
    pub fn new(id: String, addr: Addr<DashboardActor>, group_id: String, registry: GroupRegistry) -> Self {
        Self {
            hb: Instant::now(),
            id,
            addr,
            group_id,
            registry,
            self_addr: None,
        }
    }

    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(Duration::new(5, 0), |act, ctx| {
            if Instant::now().duration_since(act.hb) > Duration::new(60, 0) { // Kürzer machen nicht 60 Sekunden für den Hearbeat Ping
                println!("WebSocket connection timed out");
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("WebSocket session started");
        self.hb(ctx);

        self.self_addr = Some(ctx.address());

        {
            let mut registry = self.registry.lock().unwrap();
            let group = registry.entry(self.group_id.clone()).or_insert_with(Vec::new);
            group.push(ctx.address());
        }

        self.addr.do_send(Connect {
            addr: ctx.address(),
            connection_id: self.id.clone(),
        });
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        println!("WebSocket session stopping");

        if let Some(self_addr) = &self.self_addr {
            let mut registry = self.registry.lock().unwrap();
            if let Some(group) = registry.get_mut(&self.group_id) {
                group.retain(|addr| addr != self_addr);
                if group.is_empty() {
                    registry.remove(&self.group_id);
                }
            }
        }

        self.addr.do_send(Disconnect {
            connection_id: self.id.clone(),
        });
        Running::Stop
    }
}

impl Handler<ServerMessage> for WebSocketSession {
    type Result = ();

    fn handle(&mut self, msg: ServerMessage, ctx: &mut Self::Context) -> Self::Result {
        let response_text = serde_json::to_string(&msg).unwrap();
        ctx.text(response_text);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                let client_message: ClientMessage = match serde_json::from_str(&text) {
                    Ok(msg) => msg,
                    Err(_) => {
                        ctx.text("Invalid message format");
                        return;
                    }
                };

                println!("Client message: {:?}", client_message);

                match client_message {
                    ClientMessage::MousePosition { x, y } => {
                        let transformed_x = x * 1.1;
                        let transformed_y = y * 1.1;

                        let server_message = ServerMessage::TransformedMousePosition { x: transformed_x, y: transformed_y };

                        let registry = self.registry.clone();
                        let group_id = self.group_id.clone();
                        actix::spawn(async move {
                            let registry = registry.lock().unwrap();
                            if let Some(group) = registry.get(&group_id) {
                                for addr in group {
                                    addr.do_send(server_message.clone());
                                }
                            }
                        });
                    }
                }
                
            }
            Ok(ws::Message::Ping(msg)) => {
                ctx.pong(&msg);
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}