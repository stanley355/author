use actix_web::{get, web, HttpResponse};
use crate::bps::dynamic_data::response::NewDynamicDataResponse;
use crate::bps::request::{BpsRequestModelEnum, BpsRequestTrait};
use crate::db::PgPool;
use crate::http_error::HttpError;
use super::request::{NewDynamicDataRequest, NewDynamicDataRequestParam};

#[get("/param/data")]
async fn get_dynamic_data_service(
    pool: web::Data<PgPool>,
    query: web::Query<NewDynamicDataRequestParam>,
) -> HttpResponse {
    let param = query.into_inner();
    let result = NewDynamicDataRequest::new(BpsRequestModelEnum::Data, param).request::<NewDynamicDataResponse>().await;
    match result {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => HttpError::bad_request(&err.to_string())
    }
}

pub fn services(config: &mut web::ServiceConfig) {
    config.service(get_dynamic_data_service);
}