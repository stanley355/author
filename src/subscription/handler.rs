use super::model::Subscription;
use super::req::CreateSubscriptionPayload;
use crate::db::PgPool;
use actix_web::{
    post,
    web::{self, Json},
    HttpResponse,
};

#[post("/")]
async fn create_subscription(
    pool: web::Data<PgPool>,
    body: web::Json<CreateSubscriptionPayload>,
) -> HttpResponse {
    let existing_subscription = Subscription::check_subscription(pool.clone(), Json(body.clone()));

    match existing_subscription {
        Ok(subscription) => HttpResponse::Ok().json(subscription),
        Err(_) => {
            let new_subscription = Subscription::create(pool, body);

            match new_subscription {
                Ok(subscription) => HttpResponse::Ok().json(subscription),
                Err(err) => HttpResponse::InternalServerError().body(format!("Error : {:?}", err)),
            }
        }
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(create_subscription);
}
