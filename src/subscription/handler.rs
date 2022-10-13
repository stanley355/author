use super::model::Subscription;
use super::req::{CreateSubscriptionPayload, ViewSubscriptionQuery};
use crate::db::PgPool;
use actix_web::{
    get, post,
    web::{self, Query},
    HttpResponse,
};

#[post("/")]
async fn create_subscription(
    pool: web::Data<PgPool>,
    body: web::Json<CreateSubscriptionPayload>,
) -> HttpResponse {
    let query = ViewSubscriptionQuery {
        user_id: body.user_id.clone(),
        channels_id: body.channels_id.clone(),
    };
    let existing_subscription = Subscription::check_subscription(pool.clone(), Query(query));

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

#[get("/")]
async fn view_subscription(
    pool: web::Data<PgPool>,
    query: web::Query<ViewSubscriptionQuery>,
) -> HttpResponse {
    let existing_subscription = Subscription::check_subscription(pool.clone(), query);
    match existing_subscription {
        Ok(subscription) => HttpResponse::Ok().json(subscription),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error : {:?}", err)),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config
        .service(create_subscription)
        .service(view_subscription);
}
