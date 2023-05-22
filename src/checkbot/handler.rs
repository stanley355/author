use crate::db::PgPool;
use super::model::Checkbot;
use super::req::NewCheckbotReq;
use actix_web::{post, web, HttpResponse};

#[post("/")]
async fn add_checkbot(pool: web::Data<PgPool>, body: web::Json<NewCheckbotReq>) -> HttpResponse {
  let b = Checkbot::new(&pool, body);
  println!("ihihi");
  HttpResponse::InternalServerError().body(format!("hi"))
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(add_checkbot);
}