#![allow(unused)]
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use clap::Parser;
use log::*;
use std::io::Write;
use std::sync::Mutex;

use crate::draw::{Brush, BrushType, Color, Drawing, Point, Stroke};

mod args;
mod draw;
mod routes;
mod ws;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let args = args::Args::parse();
    let port = args.port;
    let drawing = Drawing::new(1024, 1024);
    let drawing = Data::new(Mutex::new(drawing));
    info!("Starting server on port {port}");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(args.clone()))
            .app_data(Data::clone(&drawing))
            .service(routes::pages::index)
            .service(routes::ws::accept_ws)
            .service(actix_files::Files::new("/res", "static").prefer_utf8(true))
    })
    .bind(format!("127.0.0.1:{port}"))?
    .run()
    .await?;
    info!("Server stopped");
    Ok(())
}
