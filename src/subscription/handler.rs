use super::model::Subscription;
use super::req::FindActiveSubscriptionReq;
use crate::{db::PgPool, user::res::ErrorRes};
use actix_web::{get, web, HttpResponse};

#[get("/active")]
async fn find_active_subscription(
    pool: web::Data<PgPool>,
    query: web::Query<FindActiveSubscriptionReq>,
) -> HttpResponse {
    let result = Subscription::find_active_subscription(&pool, &query.user_id);

    match result {
        Ok(topup) => HttpResponse::Ok().json(topup),
        Err(err) => HttpResponse::NotFound().json(ErrorRes {
            error: err.to_string(),
            message: "Subscription Not Found".to_string(),
        }),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(find_active_subscription);
}
