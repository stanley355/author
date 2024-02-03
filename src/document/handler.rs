use super::model::Document;
use super::req::{FindDocumentReq, NewDocumentReq};
use crate::db::PgPool;
use crate::util::web_response::WebErrorResponse;
use actix_web::http::StatusCode;
use actix_web::{get, post, web, HttpResponse};

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

#[get("")]
async fn find_documents(
    pool: web::Data<PgPool>,
    query: web::Query<FindDocumentReq>,
) -> HttpResponse {
    match query.document_id {
        Some(_) => {
            let find_result = Document::get_document(&pool, &query);
            match find_result {
                Ok(document) => HttpResponse::Ok().json(document),
                Err(err) => {
                    let error_response = WebErrorResponse {
                        status: StatusCode::BAD_REQUEST.as_u16(),
                        error: err.to_string(),
                        message: "Dokumen tidak ditemukan".to_string(),
                    };
                    HttpResponse::BadRequest().json(error_response)
                }
            }
        }
        None => {
            let find_result = Document::get_user_documents(&pool, &query.user_id);
            match find_result {
                Ok(documents) => HttpResponse::Ok().json(documents),
                Err(err) => {
                    let error_response = WebErrorResponse {
                        status: StatusCode::BAD_REQUEST.as_u16(),
                        error: err.to_string(),
                        message: "Dokumen tidak ditemukan".to_string(),
                    };
                    HttpResponse::BadRequest().json(error_response)
                }
            }
        }
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(new_document).service(find_documents);
}
