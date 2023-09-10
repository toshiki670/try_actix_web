use actix_web::{App, HttpServer};
pub mod controllers;
pub mod models;
pub mod routes;

pub mod schema;

extern crate diesel;
extern crate dotenv;

// https://crates.io/crates/simplelog
extern crate log;
extern crate simplelog;
use simplelog::*;
use std::fs::File;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("log/my_rust_binary.log").unwrap(),
        ),
    ])
    .unwrap();

    HttpServer::new(|| App::new().configure(routes::routes))
        .bind("localhost:8000")?
        .run()
        .await
}
