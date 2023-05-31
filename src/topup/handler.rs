use super::model::TopUp;
use super::req::{DokuNotifReq, TopUpReq};
use crate::user::model::User;
use crate::user::req::IncreaseBalanceReq;
use crate::{db::PgPool, user::res::ErrorRes};
use actix_web::{post, web, HttpResponse};

#[post("/")]
async fn new_topup(pool: web::Data<PgPool>, body: web::Json<TopUpReq>) -> HttpResponse {
    let result = TopUp::new(&pool, &body);

    match result {
        Ok(topup) => HttpResponse::Ok().json(topup),
        Err(err) => HttpResponse::InternalServerError().json(ErrorRes {
            error: err.to_string(),
            message: "Internal Server error".to_string(),
        }),
    }
}

#[post("/doku/notification/")]
async fn process_doku_notification(
    pool: web::Data<PgPool>,
    body: web::Json<DokuNotifReq>,
) -> HttpResponse {
    let result = TopUp::verify_topup_paid_status(&pool, &body);

    match result {
        Ok(topup) => {
            let balance_req = IncreaseBalanceReq {
                user_id: topup.user_id.to_string(),
                increase_amount: topup.topup_amount,
            };

            let balance_res = User::increase_balance(&pool, &balance_req);

            match balance_res {
                Ok(_) => HttpResponse::Ok().json(topup),
                Err(err) => HttpResponse::InternalServerError().json(ErrorRes {
                    error: err.to_string(),
                    message: "Internal Server error".to_string(),
                }),
            }
        }
        Err(err) => HttpResponse::InternalServerError().json(ErrorRes {
            error: err.to_string(),
            message: "Internal Server error".to_string(),
        }),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(new_topup).service(process_doku_notification);
}
