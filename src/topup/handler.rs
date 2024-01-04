use super::model::TopUp;
use super::req::{DokuNotifReq, TopUpReq, TopUpSubscriptionReq};
use crate::subscription::model::Subscription;
use crate::subscription::req::NewSubscriptionReq;
use crate::user::model::User;
use crate::user::req::IncreaseBalanceReq;
use crate::util::web_response::WebErrorResponse;
use crate::{db::PgPool, user::res::ErrorRes};
use actix_web::http::StatusCode;
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

#[post("/subscriptions/")]
async fn new_topup_subscriptions(
    pool: web::Data<PgPool>,
    body: web::Json<TopUpSubscriptionReq>,
) -> HttpResponse {
    let topup_result = TopUp::new_subscription(&pool, &body);

    match topup_result {
        Ok(topup) => {
            let subscription_payload = NewSubscriptionReq {
                topup_id: topup.id.clone(),
                user_id: topup.user_id.clone(),
                duration_type: body.subscription_duration_type.clone(),
            };

            let subscription_result = Subscription::new(&pool, &subscription_payload);

            match subscription_result {
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

#[post("/paylater/")]
async fn new_topup_paylater(
    pool: web::Data<PgPool>,
    body: web::Json<TopUpSubscriptionReq>,
) -> HttpResponse {
    let topup_result = TopUp::new_subscription(&pool, &body);

    match topup_result {
        Ok(topup) => {
            let subscription_payload = NewSubscriptionReq {
                topup_id: topup.id.clone(),
                user_id: topup.user_id.clone(),
                duration_type: body.subscription_duration_type.clone(),
            };

            let subscription_result = Subscription::new_paylater(&pool, &subscription_payload);

            match subscription_result {
                Ok(subscription) => HttpResponse::Created().json(subscription),
                Err(err) => {
                    let web_res = WebErrorResponse {
                        status: StatusCode::BAD_REQUEST.as_u16(),
                        error: err.to_string(),
                        message: "Fail to create subscription".to_string(),
                    };
                    HttpResponse::BadRequest().json(web_res)
                }
            }
        }
        Err(err) => {
            let web_res = WebErrorResponse {
                status: StatusCode::BAD_REQUEST.as_u16(),
                error: err.to_string(),
                message: "Fail to create topup".to_string(),
            };
            HttpResponse::BadRequest().json(web_res)
        }
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config
        .service(new_topup)
        .service(process_doku_notification)
        .service(new_topup_subscriptions)
        .service(new_topup_paylater);
}
