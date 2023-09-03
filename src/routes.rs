use actix_web::{web, HttpRequest, HttpResponse, Responder};
use crate::controllers::*;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index::index));
    cfg.route("/create_user", web::post().to(users::create));
}