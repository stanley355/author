use actix_web::{post, web, HttpResponse};

use super::model::Subscription;
use super::request::{NewSubscriptionRequest, DokuNotificationRequest};
use crate::{db::PgPool, http_error::HttpError};

#[post("/")]
async fn post_subscription(
    pool: web::Data<PgPool>,
    json_request: web::Json<NewSubscriptionRequest>,
) -> HttpResponse {
    let request = json_request.into_inner();
    let subscription_result = Subscription::new_insert(&pool, &request, false);
    match subscription_result {
        Ok(subscription) => HttpResponse::Created().json(subscription),
        Err(diesel_error) => HttpError::internal_server_error(&diesel_error.to_string()),
    }
}

#[post("/doku/notification/")]
async fn post_doku_notificatioin(
    pool: web::Data<PgPool>,
    json_request: web::Json<DokuNotificationRequest>,
) -> HttpResponse {
    let request = json_request.into_inner();
    // let subscription_result = Subscription::new_insert(&pool, &request, false);
    // match subscription_result {
    //     Ok(subscription) => HttpResponse::Created().json(subscription),
    //     Err(diesel_error) => HttpError::internal_server_error(&diesel_error.to_string()),
    // }
    HttpResponse::Ok().body("paid")
}

pub fn services(config: &mut web::ServiceConfig) {
    config.service(post_subscription);
}
