use super::model::Subscription;
use super::req::NewSubscriptionReq;
use crate::{db::PgPool, user::res::ErrorRes};
use actix_web::{post, web, HttpResponse};

#[post("/")]
async fn new(pool: web::Data<PgPool>, body: web::Json<NewSubscriptionReq>) -> HttpResponse {
    let result = Subscription::new(&pool, &body);

    match result {
        Ok(subscription) => HttpResponse::Ok().json(subscription),
        Err(err) => HttpResponse::InternalServerError().json(ErrorRes {
            error: err.to_string(),
            message: "Internal Server error".to_string(),
        }),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(new);
}
