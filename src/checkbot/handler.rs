use crate::db::PgPool;
use super::req::NewCheckbotReq;
use actix_web::{post, web, HttpResponse};

#[post("/")]
async fn new(pool: web::Data<PgPool>, body: web::Json<NewCheckbotReq>) -> HttpResponse {
  HttpResponse::Ok().body(format!("hi"))
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(new);
}