use actix_web::web;
use diesel::{ExpressionMethods, QueryResult, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::db::PgPool;
use crate::schema::documents;

use super::req::CreateDocumentReq;

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
}
