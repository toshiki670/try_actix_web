extern crate diesel;

use crate::models::user::User;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use simplelog::*;

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
