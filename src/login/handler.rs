use crate::db::PgPool;
use actix_web::{get, post, put, web, HttpResponse};

#[get("/")]
async fn login(pool: web::Data<PgPool>) -> HttpResponse {
  HttpResponse::BadRequest().body(format!("Missing Parameter: code, year"))
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(login);
}