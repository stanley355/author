use super::model::Checkbot;
use super::req::NewCheckbotReq;
use crate::{db::PgPool, user::res::ErrorRes};
use actix_web::{post, web, HttpResponse};

#[post("/")]
async fn add_checkbot(pool: web::Data<PgPool>, body: web::Json<NewCheckbotReq>) -> HttpResponse {
    let result = Checkbot::new(&pool, body);

    match result {
        Ok(checkbot) => HttpResponse::Accepted().json(checkbot),
        Err(err) => HttpResponse::InternalServerError().json(ErrorRes {
            error: err.to_string(),
            message: "Internal Server error".to_string(),
        }),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(add_checkbot);
}
