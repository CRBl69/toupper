use std::sync::{Arc, Mutex};

use actix_web::{
    get,
    web::{self, Data},
    App, Error, HttpRequest, HttpResponse, HttpServer,
};
use actix_web_actors::ws;
use log::*;

use crate::{draw::Drawing, ws::handler::WebSocketHandler, AppData};

#[get("/ws")]
pub async fn accept_ws(
    req: HttpRequest,
    stream: web::Payload,
    data: Data<AppData<'static>>,
) -> Result<HttpResponse, Error> {
    let resp = ws::start(
        WebSocketHandler {
            drawing: Arc::clone(&data.drawing),
        },
        &req,
        stream,
    );
    info!("New websocket client connected");
    resp
}
