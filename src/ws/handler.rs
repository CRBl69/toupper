use std::sync::Mutex;

use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use log::*;

use crate::{draw::Drawing, ws::messages::Message};

pub struct WebSocketHandler<'a> {
    drawing: Mutex<Drawing<'a>>,
}

impl Actor for WebSocketHandler<'static> {
    type Context = ws::WebsocketContext<Self>;
}

impl WebSocketHandler<'_> {
    fn handle_message(&mut self, msg: &str) {
        info!("Incomming websocket message: {msg}");
        if let Ok(msg) = serde_json::from_str::<Message>(msg) {
            match msg {
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
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketHandler<'static> {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => self.handle_message(&text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}
