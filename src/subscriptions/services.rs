use actix_web::{post, web, HttpResponse};

use super::model::Subscription;
use super::request::NewSubscriptionRequest;
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

pub fn services(config: &mut web::ServiceConfig) {
    config.service(post_subscription);
}
