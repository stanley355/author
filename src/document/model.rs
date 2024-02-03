use actix_web::web;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
use serde::{Deserialize, Serialize};

use crate::{db::PgPool, schema::documents};

use super::req::{DeleteDocumentReq, FindDocumentReq, NewDocumentReq, UpdateDocumentReq};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Document {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub name: String,
    pub content: Option<String>,
    pub ai_completion: Option<String>,
}

impl Document {
    pub fn new(pool: &web::Data<PgPool>, body: &NewDocumentReq) -> QueryResult<Document> {
        let conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&body.user_id).unwrap();
        let data = (
            (documents::user_id.eq(uuid)),
            (documents::name.eq(&body.name)),
        );

        diesel::insert_into(documents::table)
            .values(data)
            .get_result(&conn)
    }

    pub fn get_user_documents(
        pool: &web::Data<PgPool>,
        user_id: &str,
    ) -> QueryResult<Vec<Document>> {
        let conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&user_id).unwrap();
        documents::table
            .filter(documents::user_id.eq(uuid))
            .order_by(documents::updated_at.desc())
            .get_results(&conn)
    }

    pub fn get_document(
        pool: &web::Data<PgPool>,
        query: &FindDocumentReq,
    ) -> QueryResult<Document> {
        let conn = pool.get().unwrap();
        let user_id = uuid::Uuid::parse_str(&query.user_id).unwrap();
        let document_id = uuid::Uuid::parse_str(&query.document_id.as_ref().unwrap()).unwrap();
        documents::table
            .filter(
                documents::user_id
                    .eq(user_id)
                    .and(documents::id.eq(document_id)),
            )
            .get_result(&conn)
    }

    pub fn update_document(
        pool: &web::Data<PgPool>,
        body: &web::Json<UpdateDocumentReq>,
    ) -> QueryResult<Document> {
        let conn = pool.get().unwrap();
        let document_id = uuid::Uuid::parse_str(&body.id).unwrap();
        let user_id = uuid::Uuid::parse_str(&body.user_id).unwrap();

        let data = (
            (documents::id.eq(&document_id)),
            (documents::user_id.eq(&user_id)),
            (documents::name.eq(&body.name)),
            (documents::content.eq(&body.content)),
            (documents::ai_completion.eq(&body.ai_completion)),
        );

        diesel::update(documents::table)
            .filter(
                documents::id
                    .eq(document_id)
                    .and(documents::user_id.eq(user_id)),
            )
            .set(data)
            .get_result(&conn)
    }

    pub fn delete(pool: &web::Data<PgPool>, query: &DeleteDocumentReq) -> QueryResult<Document> {
        let conn = pool.get().unwrap();
        let document_id = uuid::Uuid::parse_str(&query.document_id).unwrap();
        let user_id = uuid::Uuid::parse_str(&query.user_id).unwrap();

        diesel::delete(documents::table)
            .filter(
                documents::id
                    .eq(document_id)
                    .and(documents::user_id.eq(user_id)),
            )
            .get_result(&conn)
    }
}
