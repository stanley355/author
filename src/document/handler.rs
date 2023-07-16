use super::model::Document;
use super::req::CreateDocumentReq;
use crate::{db::PgPool, user::res::ErrorRes};
use actix_web::{post, web, HttpResponse};

#[post("/")]
async fn new_document(pool: web::Data<PgPool>, body: web::Json<CreateDocumentReq>) -> HttpResponse {
    let document = Document::new(&pool, body);

    match document {
        Ok(doc) => HttpResponse::Ok().json(doc),
        Err(err) => HttpResponse::InternalServerError().json(ErrorRes {
            error: err.to_string(),
            message: "Something went wrong, please try again".to_string(),
        }),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(new_document);
}
