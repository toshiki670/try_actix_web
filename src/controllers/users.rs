extern crate diesel;
use crate::controllers::ErrorResponse;

use crate::models::user::User;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use simplelog::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ShowParams {
    id: i32,
}
pub async fn show(path: web::Path<ShowParams>) -> HttpResponse {
    info!("[BEGIN] show: id: {:?}", &path);

    let res = match User::find(path.id) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::Ok().json(ErrorResponse {
            message: err.to_string(),
        }),
    };

    info!("[END] show");

    res
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateParams {
    name: String,
}
pub async fn create(item: web::Json<CreateParams>) -> HttpResponse {
    info!("[BEGIN] create");

    let res = match User::create(&(item.name)) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::Ok().json(ErrorResponse {
            message: err.to_string(),
        }),
    };

    info!("[END] create");

    res
}
