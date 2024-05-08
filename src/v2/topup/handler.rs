use actix_web::{post, web, HttpResponse};

use super::model::TopUp;
use super::request::{TopupPayasyougoRequestBody, TopupPremiumRequestBody};
use crate::v2::topup::doku::DokuNotifRequestBody;
use crate::v2::user::model::User;
use crate::{db::PgPool, v2::http_error_response::HttpErrorResponse};

#[post("/payasyougo/")]
async fn new_topup_payasyougo(
    pool: web::Data<PgPool>,
    body: web::Json<TopupPayasyougoRequestBody>,
) -> HttpResponse {
    if &body.amount < &10000.0 {
        return HttpErrorResponse::bad_request("Minimum amount is Rp10.000".to_string());
    }

    let topup_result = TopUp::new_payasyougo(&pool, &body);

    match topup_result {
        Ok(topup) => HttpResponse::Ok().json(topup),
        Err(err) => HttpErrorResponse::internal_server_error(err.to_string()),
    }
}

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

            let msg = "Fail to update user balance or subscription".to_string();
            return HttpErrorResponse::internal_server_error(msg);
        }
        Err(err) => HttpErrorResponse::internal_server_error(err.to_string()),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config
        .service(new_topup_payasyougo)
        .service(new_topup_premium)
        .service(new_doku_notification);
}
