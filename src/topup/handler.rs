use super::model::TopUp;
use super::req::{TopupPaidReq, TopupPayasyougoReq, TopupPremiumReq};
use crate::subscription::model::Subscription;
use crate::util::web_response::WebErrorResponse;
use crate::{db::PgPool, user::model::User};
use actix_web::{post, web, HttpResponse};

#[post("/payasyougo/")]
async fn new_topup_payasyougo(
    pool: web::Data<PgPool>,
    body: web::Json<TopupPayasyougoReq>,
) -> HttpResponse {
    let result = TopUp::new_payasyougo(&pool, &body);

    match result {
        Ok(topup) => HttpResponse::Ok().json(topup),
        Err(err) => {
            let err_res =
                WebErrorResponse::server_error(err, "Fail to create topup, please try again");
            HttpResponse::InternalServerError().json(err_res)
        }
    }
}

#[post("/premium/")]
async fn new_topup_premium(
    pool: web::Data<PgPool>,
    body: web::Json<TopupPremiumReq>,
) -> HttpResponse {
    let result = TopUp::new_premium(&pool, &body);

    match result {
        Ok(topup) => {
            let subscription_result = Subscription::new(&pool, &body, &topup);

            match subscription_result {
                Ok(_) => {
                    return HttpResponse::Ok().json(topup);
                }
                Err(err) => {
                    let err_res = WebErrorResponse::server_error(
                        err,
                        "Fail to create subscription, please try again",
                    );
                    return HttpResponse::InternalServerError().json(err_res);
                }
            }
        }
        Err(err) => {
            let err_res =
                WebErrorResponse::server_error(err, "Fail to create topup, please try again");
            HttpResponse::InternalServerError().json(err_res)
        }
    }
}

#[post("/paid/")]
async fn new_paid_topup(pool: web::Data<PgPool>, body: web::Json<TopupPaidReq>) -> HttpResponse {
    let result = TopUp::update_paid_topup(&pool, &body);

    match result {
        Ok(topup) => {
            if topup.topup_type == "topup".to_string() {
                let update_user_balance =
                    User::increase_balance(&pool, topup.user_id, topup.topup_amount);

                match update_user_balance {
                    Ok(_) => {
                        return HttpResponse::Ok().json(topup);
                    }
                    Err(err) => {
                        let err_res = WebErrorResponse::server_error(
                            err,
                            "Fail to update balance, please try again",
                        );
                        return HttpResponse::InternalServerError().json(err_res);
                    }
                }
            }
            HttpResponse::Ok().json(topup)
        }
        Err(err) => {
            let err_res =
                WebErrorResponse::server_error(err, "Fail to update payment, please try again");
            HttpResponse::InternalServerError().json(err_res)
        }
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config
        .service(new_topup_payasyougo)
        .service(new_topup_premium)
        .service(new_paid_topup);
}
