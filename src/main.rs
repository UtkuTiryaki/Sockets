use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix::prelude::*;
use actix_web_actors::ws;
use uuid::Uuid;

mod actors;
mod messages;
mod session;

use actors::DashboardActor;
use session::WebSocketSession;

async fn ws_handler(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<Addr<DashboardActor>>,
) -> Result<HttpResponse, Error> {
    let connection_id = Uuid::new_v4().to_string();
    ws::start(WebSocketSession::new(connection_id, data.get_ref().clone()), &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let dashboard_actor = DashboardActor::new().start();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(dashboard_actor.clone()))
            .route("/ws/", web::get().to(ws_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}