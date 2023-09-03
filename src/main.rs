use actix_web::{App, HttpServer};
pub mod controllers;
pub mod models;
pub mod routes;

pub mod schema;

extern crate diesel;
extern crate dotenv;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(routes::routes))
        .bind("localhost:8000")?
        .run()
        .await
}
