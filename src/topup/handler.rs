use super::model::TopUp;
use super::req::TopupPaidReq;
use crate::db::PgPool;
use crate::topup::req::TopupPayasyougoReq;
use crate::util::web_response::WebErrorResponse;

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
            let err_res = WebErrorResponse::server_error(err, "Fail to create, please try again");
            HttpResponse::InternalServerError().json(err_res)
        }
    }
}

#[post("/paid/")]
async fn new_paid_topup(pool: web::Data<PgPool>, body: web::Json<TopupPaidReq>) -> HttpResponse {
    let result = TopUp::update_paid_topup(&pool, &body);

    match result {
        Ok(topup) => {
            
            HttpResponse::Ok().json(topup)
        },
        Err(err) => {
            let err_res = WebErrorResponse::server_error(err, "Fail to update, please try again");
            HttpResponse::InternalServerError().json(err_res)
        }
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(new_topup_payasyougo).service(new_paid_topup);
}
