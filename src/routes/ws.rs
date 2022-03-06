use std::sync::Mutex;

use actix_web::{
    get,
    web::{self, Data},
    App, Error, HttpRequest, HttpResponse, HttpServer,
};
use actix_web_actors::ws;
use log::*;

use crate::{draw::Drawing, ws::handler::WebSocketHandler};

#[get("/ws")]
pub async fn accept_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(WebSocketHandler { drawing }, &req, stream);
    info!("New websocket client connected");
    resp
}
