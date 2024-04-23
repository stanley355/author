use actix_web::{get, post, web, HttpResponse};

use crate::db::PgPool;

#[get("")]
async fn get_user(pool: web::Data<PgPool>) -> HttpResponse {
    HttpResponse::Ok().body("".to_string())
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(get_user);
}
