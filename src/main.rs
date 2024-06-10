use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix::prelude::*;
use actix_web_actors::ws;
use uuid::Uuid;

mod actors;
mod messages;
mod session;

use actors::DashboardActor;
use session::{GroupRegistry, WebSocketSession};

async fn ws_handler(
    req: HttpRequest,
    stream: web::Payload,
    dashboard_addr: web::Data<Addr<DashboardActor>>,
    group_registry: web::Data<GroupRegistry>,
    path: web::Path<String>
) -> Result<HttpResponse, Error> {
    let connection_id = Uuid::new_v4().to_string();
    let group_id = path.into_inner();

    ws::start(
        WebSocketSession::new(
            connection_id,
            dashboard_addr.get_ref().clone(),
            group_id,
            group_registry.get_ref().clone()
        ),
        &req,
        stream
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let dashboard_actor = DashboardActor::new().start();
    let group_registry = GroupRegistry::default();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(dashboard_actor.clone()))
            .app_data(web::Data::new(group_registry.clone()))   
            .route("/ws/{group_id}", web::get().to(ws_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}