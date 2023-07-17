use super::model::Document;
use super::req::{CreateDocumentReq, FindDocumentsReq, DeleteDocumentReq};
use crate::{db::PgPool, user::res::ErrorRes};
use actix_web::{get, post, web, HttpResponse, delete};

#[get("")]
async fn find_document(
    pool: web::Data<PgPool>,
    query: web::Query<FindDocumentsReq>,
) -> HttpResponse {
    let user_id = query.user_id.clone();
    let document = Document::find_by_user_id(&pool, &user_id);

    match document {
        Ok(doc) => HttpResponse::Ok().json(doc),
        Err(err) => HttpResponse::InternalServerError().json(ErrorRes {
            error: err.to_string(),
            message: "Document not found, please try again".to_string(),
        }),
    }
}

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

#[delete("")]
async fn delete_document(pool: web::Data<PgPool>, query: web::Query<DeleteDocumentReq>) -> HttpResponse {
    let id = query.id.clone();
    let document = Document::delete(&pool, &id);

    match document {
        Ok(doc) => HttpResponse::Ok().json(doc),
        Err(err) => HttpResponse::InternalServerError().json(ErrorRes {
            error: err.to_string(),
            message: "Something went wrong, please try again".to_string(),
        }),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(new_document).service(find_document).service(delete_document);
}
