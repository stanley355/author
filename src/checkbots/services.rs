use actix_web::{HttpResponse, post, web};
use crate::checkbots::request::NewCheckbotRequest;
use crate::db::PgPool;

#[post("/")]
pub async fn new_checkbot_service(
    pool: web::Data<PgPool>,
    request_json: web::Json<NewCheckbotRequest>
) -> HttpResponse {
    HttpResponse::Ok().body("hhi")
}

pub fn services(config: &mut web::ServiceConfig) {
    config.service(new_checkbot_service);
}