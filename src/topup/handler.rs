use super::model::TopUp;
use super::req::TopUpReq;
use crate::{db::PgPool, user::res::ErrorRes};
use actix_web::{post, web, HttpResponse};

#[post("/")]
async fn new_topup(pool: web::Data<PgPool>, body: web::Json<TopUpReq>) -> HttpResponse {
    let result = TopUp::new(&pool, &body);

    match result {
        Ok(topup) => HttpResponse::Ok().json(topup),
        Err(err) => HttpResponse::InternalServerError().json(ErrorRes {
            error: err.to_string(),
            message: "Internal Server error".to_string(),
        }),
    }
}

#[post("/doku/notification/")]
async fn process_doku_notification(pool: web::Data<PgPool>, body: web::Json<TopUpReq>) -> HttpResponse {
    let result = TopUp::new(&pool, &body);

    match result {
        Ok(topup) => HttpResponse::Ok().json(topup),
        Err(err) => HttpResponse::InternalServerError().json(ErrorRes {
            error: err.to_string(),
            message: "Internal Server error".to_string(),
        }),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(new_topup);
}
