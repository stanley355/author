use actix_web::{post, web, HttpResponse};

use super::model::Subscription;
use super::request::{DokuNotificationRequest, NewSubscriptionRequest};
use crate::students::Student;
use crate::{db::PgPool, http_error::HttpError};

#[post("/")]
async fn post_subscription(
    pool: web::Data<PgPool>,
    json_request: web::Json<NewSubscriptionRequest>,
) -> HttpResponse {
    let request = json_request.into_inner();
    let user_id = uuid::Uuid::parse_str(&request.user_id).unwrap();

    let student_result = Student::find_user_last_active_application(&pool, &user_id);
    let is_student = match student_result {
        Ok(_) => true,
        Err(_) => false,
    };

    let subscription_result = Subscription::new_insert(&pool, &request, is_student);
    match subscription_result {
        Ok(subscription) => HttpResponse::Created().json(subscription),
        Err(diesel_error) => HttpError::internal_server_error(&diesel_error.to_string()),
    }
}

#[post("/doku/notification/")]
async fn post_doku_notification(
    pool: web::Data<PgPool>,
    json_request: web::Json<DokuNotificationRequest>,
) -> HttpResponse {
    let request = json_request.into_inner();

    let subscription_update_result =
        Subscription::update_paid(&pool, &request.transaction.original_request_id);

    match subscription_update_result {
        Ok(subscription) => HttpResponse::Ok().json(subscription),
        Err(diesel_error) => HttpError::internal_server_error(&diesel_error.to_string()),
    }
}

pub fn services(config: &mut web::ServiceConfig) {
    config
        .service(post_subscription)
        .service(post_doku_notification);
}
