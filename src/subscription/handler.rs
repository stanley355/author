use super::model::Subscription;
use super::req::CreateSubscriptionPayload;
use crate::db::PgPool;
use actix_web::{post, web, HttpResponse};

#[post("/")]
async fn create_subscription(
    pool: web::Data<PgPool>,
    body: web::Json<CreateSubscriptionPayload>,
) -> HttpResponse {
    let new_subscription = Subscription::create(pool, body);

    match new_subscription {
        Ok(subscription) => HttpResponse::Ok().json(subscription),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error : {:?}", err)),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(create_subscription);
}
