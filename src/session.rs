
use actix::prelude::*;
use actix_web_actors::ws;
use serde_json;
use std::time::{Duration, Instant};
use crate::actors::DashboardActor;
use crate::messages::{ClientMessage, Connect, Disconnect, GroupMessage, JoinGroup, LeaveGroup, SendTextMessage, ServerMessage};

pub struct WebSocketSession {
    hb: Instant,
    id: String,
    addr: Addr<DashboardActor>,
}

impl WebSocketSession {
    pub fn new(id: String, addr: Addr<DashboardActor>) -> Self {
        Self {
            hb: Instant::now(),
            id,
            addr,
        }
    }

    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(Duration::new(5, 0), |act, ctx| {
            if Instant::now().duration_since(act.hb) > Duration::new(60, 0) {
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

        self.addr.do_send(Connect {
            addr: ctx.address(),
            connection_id: self.id.clone(),
        });
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        println!("WebSocket session stopping");
        self.addr.do_send(LeaveGroup {
            group_id: "default".into(),
            connection_id: self.id.clone(),
        });
        self.addr.do_send(Disconnect {
            connection_id: self.id.clone(),
        });
        Running::Stop
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

                // For demonstration, every message joins the default group
                self.addr.do_send(JoinGroup {
                    group_id: "default".into(),
                    connection_id: self.id.clone(),
                });

                let server_message = ServerMessage {
                    message: format!("Echo: {}", client_message.message),
                };
                let response_text = serde_json::to_string(&server_message).unwrap();

                // Broadcast the message to the group
                self.addr.do_send(GroupMessage {
                    group_id: "default".into(),
                    message: response_text,
                });
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

impl Handler<SendTextMessage> for WebSocketSession {
    type Result = ();

    fn handle(&mut self, msg: SendTextMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.message);
    }
}