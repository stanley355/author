use actix_web::{post, web, HttpResponse};

use super::model::TopUp;
use super::request::TopupPremiumRequestBody;
use crate::v2::subscription::model::Subscription;
use crate::v2::topup::doku::DokuNotifRequestBody;
use crate::v2::user::model::User;
use crate::{db::PgPool, v2::http_error_response::HttpErrorResponse};

#[post("/premium/")]
async fn new_topup_premium(
    pool: web::Data<PgPool>,
    body: web::Json<TopupPremiumRequestBody>,
) -> HttpResponse {
    let topup_result = TopUp::new_premium(&pool, &body);

    match topup_result {
        Ok(topup) => HttpResponse::Ok().json(topup),
        Err(err) => HttpErrorResponse::internal_server_error(err.to_string()),
    }
}

#[post("/doku/notification/")]
async fn new_doku_notification(
    pool: web::Data<PgPool>,
    body: web::Json<DokuNotifRequestBody>,
) -> HttpResponse {
    let topup_update_result =
        TopUp::update_paid_topup(&pool, &body.transaction.original_request_id);

    match topup_update_result {
        Ok(topup) => {
            if &topup.topup_type == "topup" {
                let user_result =
                    User::increase_balance(&pool, &topup.user_id.to_string(), &topup.topup_amount);

                return match user_result {
                    Ok(_) => HttpResponse::Ok().json(topup),
                    Err(_) => HttpErrorResponse::internal_server_error(
                        "Fail to update user balance".to_string(),
                    ),
                };
            }

            if &topup.topup_type == "subscription" {
                let subscription_result =
                    Subscription::update_paid_subscription(&pool, &topup.id.to_string());
                return match subscription_result {
                    Ok(_) => HttpResponse::Ok().json(topup),
                    Err(_) => HttpErrorResponse::internal_server_error(
                        "Fail to update subscription".to_string(),
                    ),
                };
            }

            let msg = "Fail to update user balance or subscription".to_string();
            return HttpErrorResponse::internal_server_error(msg);
        }
        Err(err) => HttpErrorResponse::internal_server_error(err.to_string()),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config
        .service(new_topup_premium)
        .service(new_doku_notification);
}
