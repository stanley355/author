use super::model::Document;
use super::req::NewDocumentReq;
use crate::db::PgPool;
use crate::util::web_response::WebErrorResponse;
use actix_web::http::StatusCode;
use actix_web::{post, web, HttpResponse};

#[post("/")]
async fn new_document(pool: web::Data<PgPool>, body: web::Json<NewDocumentReq>) -> HttpResponse {
    let result = Document::new(&pool, &body);

    match result {
        Ok(document) => HttpResponse::Ok().json(document),
        Err(err) => {
            let error_response = WebErrorResponse {
                status: StatusCode::BAD_REQUEST.as_u16(),
                error: err.to_string(),
                message: "Gagal Membuat Dokumen".to_string(),
            };
            HttpResponse::InternalServerError().json(error_response)
        }
    }
}
pub fn route(config: &mut web::ServiceConfig) {
    config.service(new_document);
}
