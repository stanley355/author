use super::model::Subscription;
use super::req::{
    CreateSubscriptionPayload, UpdateSubscriptionChannelPayload, ViewSubscriptionPayload,
};
use crate::db::PgPool;

use actix_web::{get, post, put, web, HttpResponse};

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
async fn view_subscription(
    pool: web::Data<PgPool>,
    query: web::Query<ViewSubscriptionPayload>,
) -> HttpResponse {
    let user_subscriptions = Subscription::check_subscriptions(pool.clone(), query);
    match user_subscriptions {
        Ok(subscriptions) => HttpResponse::Ok().json(subscriptions),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error : {:?}", err)),
    }
}

#[put("/channels/")]
async fn update_subscription_channels_data(
    pool: web::Data<PgPool>,
    body: web::Json<UpdateSubscriptionChannelPayload>,
) -> HttpResponse {
    let subscriptions_update = Subscription::update_subscription_channels(pool, body);

    match subscriptions_update {
        Ok(subscriptions) => HttpResponse::Ok().json(subscriptions),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error : {:?}", err)),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config
        .service(create_subscription)
        .service(view_subscription)
        .service(update_subscription_channels_data);
}
