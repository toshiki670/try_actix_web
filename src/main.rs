pub mod controllers;
pub mod routes;

pub mod models;
pub mod schema;

use actix_web::{App, HttpServer};

// https://crates.io/crates/simplelog
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
