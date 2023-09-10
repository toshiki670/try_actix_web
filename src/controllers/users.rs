extern crate diesel;
use crate::controllers::ErrorResponse;

use crate::models::user::User;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use simplelog::*;

pub async fn show(path: web::Path<(i32)>) -> HttpResponse {
    let id = path.into_inner();
    info!("[BEGIN] show: id: {}", id);

    let res = match User::find(id) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::Ok().json(ErrorResponse {
            message: err.to_string(),
        }),
    };

    info!("[END] show");

    res
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    name: String,
}

pub async fn create(item: web::Json<UserData>) -> HttpResponse {
    info!("[BEGIN] create");
    let user = User::create(&(item.name));
    info!("Created user info: {:?}", user);

    info!("[END] create");
    HttpResponse::Created().body("Inserting")
}
