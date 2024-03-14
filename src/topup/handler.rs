use super::model::TopUp;
use super::req::DokuNotifReq;
use crate::subscription::model::Subscription;
use crate::topup::req::TopupPayasyougoReq;
use crate::user::model::User;
use crate::user::req::IncreaseBalanceReq;
use crate::{db::PgPool, user::res::ErrorRes};

use actix_web::{post, web, HttpResponse};

#[post("/payasyougo/")]
async fn new_topup_payasyougo(
    pool: web::Data<PgPool>,
    body: web::Json<TopupPayasyougoReq>,
) -> HttpResponse {
    let result = TopUp::new_payasyougo(&pool, &body);


    
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
    let topup_result = TopUp::verify_doku_paid_status(&pool, &body);

    match topup_result {
        Ok(topup) => {
            if topup.topup_type == "subscription" {
                let subcription_res = Subscription::verify_subscription_paid_status(&pool, &body);

                match subcription_res {
                    Ok(sub) => HttpResponse::Ok().json(sub),
                    Err(err) => HttpResponse::InternalServerError().json(ErrorRes {
                        error: err.to_string(),
                        message: "Internal Server error".to_string(),
                    }),
                }
            } else {
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
        }
        Err(err) => HttpResponse::InternalServerError().json(ErrorRes {
            error: err.to_string(),
            message: "Internal Server error".to_string(),
        }),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config
        .service(new_topup_payasyougo)
        .service(process_doku_notification);
}
