use crate::db::PgPool;
use actix_web::{post, web, HttpResponse};

#[post("/")]
async fn new_prompt(pool: web::Data<PgPool>) -> HttpResponse {
    HttpResponse::Ok().body("".to_string())
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(new_prompt);
}
