#![allow(unused)]
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use clap::Parser;
use log::*;
use std::io::Write;
use std::sync::{Arc, Mutex};

use crate::draw::{Brush, BrushType, Color, Drawing, Point, Stroke};

mod args;
mod draw;
mod routes;
mod ws;

#[derive(Clone)]
pub struct AppData<'a> {
    pub drawing: Arc<Mutex<Drawing<'a>>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let args = args::Args::parse();
    let port = args.port;
    let drawing = Drawing::new(1024, 1024);
    let drawing = Mutex::new(drawing);
    let app_data = AppData {
        drawing: Arc::new(drawing),
    };
    info!("Starting server on port {port}");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(args.clone()))
            .app_data(Data::new(app_data.clone()))
            .service(routes::pages::index)
            .service(routes::ws::accept_ws)
    })
    .bind(format!("127.0.0.1:{port}"))?
    .run()
    .await?;
    info!("Server stopped");
    Ok(())
}
