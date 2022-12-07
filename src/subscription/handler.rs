use super::model::Subscription;
use super::req::{CreateSubscriptionPayload, FindSubscriptionQuery};
use crate::db::PgPool;

use actix_web::{get, post, web, HttpResponse};

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

#[get("/")]
async fn find_subscriptions(
    pool: web::Data<PgPool>,
    query: web::Query<FindSubscriptionQuery>,
) -> HttpResponse {
    let user_subscriptions = Subscription::find_subscriptions(pool.clone(), query.user_id.clone());
    match user_subscriptions {
        Ok(subscriptions) => HttpResponse::Ok().json(subscriptions),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error : {:?}", err)),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config
        .service(create_subscription)
        .service(find_subscriptions);
}
