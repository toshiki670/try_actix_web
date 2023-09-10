use crate::controllers::*;
use actix_web::{web, HttpRequest, HttpResponse, Responder};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index::index));

    // Users
    cfg.route("/users/{id}", web::get().to(users::show));
    cfg.route("/users", web::post().to(users::create));
}
