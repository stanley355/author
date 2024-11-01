
use actix_web::{get, web, HttpResponse};
use crate::bps::request::{BpsRequestModelEnum, BpsRequestTrait};
use crate::db::PgPool;
use super::request::{NewDynamicDataRequest, NewDynamicDataRequestParam};

#[get("/param/var")]
async fn get_dynamic_var_service(
    pool: web::Data<PgPool>,
    query: web::Query<NewDynamicDataRequestParam>
) -> HttpResponse {
    let param = query.into_inner();
    let endpoint = NewDynamicDataRequest::new(BpsRequestModelEnum::Data, param).create_request_url();
    HttpResponse::Ok().body(endpoint)
}

pub fn services(config: &mut web::ServiceConfig) {
    config.service(get_dynamic_var_service);
}