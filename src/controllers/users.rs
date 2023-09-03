extern crate diesel;

use crate::models::user::User;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    name: String,
}

pub async fn create(item: web::Json<UserData>) -> HttpResponse {
    let user = User::create(&(item.name));
    println!("{:?}",user);
    HttpResponse::Created().body("Inserting")

}