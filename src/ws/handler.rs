use std::sync::{Arc, Mutex};

use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use log::*;

use crate::{draw::Drawing, ws::messages::Message};

pub struct WebSocketHandler {
    pub drawing: Arc<Mutex<Drawing<'static>>>,
}

impl Actor for WebSocketHandler {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketHandler {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                info!("Incomming websocket message: {text}");
                if let Ok(m) = serde_json::from_str::<Message>(&text) {
                    match m {
                        Message::Stroke(stroke) => {
                            info!("Stroke");
                        }
                        Message::Image(image) => {
                            info!("Image");
                        }
                        Message::Motion(stroke) => {
                            info!("Motion");
                        }
                    };
                } else {
                    error!("Could not parse message");
                };
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}
