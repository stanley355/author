use actix_web::web;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::db::PgPool;
use crate::schema::documents;

use super::req::{CreateDocumentReq, UpdateDocumentReq};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Document {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub name: String,
    pub doc_type: String,
}

impl Document {
    pub fn new(
        pool: &web::Data<PgPool>,
        body: web::Json<CreateDocumentReq>,
    ) -> QueryResult<Document> {
        let conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&body.user_id).unwrap();

        let data = (
            (documents::user_id.eq(uuid)),
            (documents::name.eq(&body.name)),
            (documents::doc_type.eq(body.doc_type.to_string())),
        );

        diesel::insert_into(documents::table)
            .values(data)
            .get_result(&conn)
    }

    pub fn find_by_user_id(
        pool: &web::Data<PgPool>,
        user_id: &String,
    ) -> QueryResult<Vec<Document>> {
        let conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&user_id).unwrap();
        documents::table
            .filter(documents::user_id.eq(uuid))
            .order_by(documents::created_at.desc())
            .get_results::<Document>(&conn)
    }

    pub fn delete(pool: &web::Data<PgPool>, doc_id: &String) -> QueryResult<Document> {
        let conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&doc_id).unwrap();

        diesel::delete(documents::table)
            .filter(documents::id.eq(uuid))
            .get_result::<Document>(&conn)
    }

    pub fn update_name(
        pool: &web::Data<PgPool>,
        body: &web::Json<UpdateDocumentReq>,
    ) -> QueryResult<Document> {
        let conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&body.id).unwrap();

        diesel::update(documents::table)
            .filter(documents::id.eq(uuid))
            .set(documents::name.eq(&body.name))
            .get_result::<Document>(&conn)
    }
}
