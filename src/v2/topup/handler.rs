use actix_web::{post, web, HttpResponse};

use crate::{db::PgPool, v2::http_error_response::HttpErrorResponse};

use super::{model::TopUp, request::TopupPayasyougoRequestBody};

#[post("/payasyougo/")]
async fn new_topup_payasyougo(
    pool: web::Data<PgPool>,
    body: web::Json<TopupPayasyougoRequestBody>,
) -> HttpResponse {
    if &body.amount < &10000.0 {
      return HttpErrorResponse::bad_request("Minimum amount is Rp10.000".to_string())
    }

    let topup_result = TopUp::new_payasyougo(&pool, &body);

    match topup_result {
        Ok(topup) => HttpResponse::Ok().json(topup),
        Err(err) => HttpErrorResponse::internal_server_error(err.to_string()),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(new_topup_payasyougo);
}
