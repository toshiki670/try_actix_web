use actix_web::{web, HttpRequest, HttpResponse, Responder};
use simplelog::*;

pub async fn index() -> impl Responder {
    info!("[BEGIN] index");
    info!("[END] index");
    HttpResponse::Ok().body("Hello world!")
}
