use actix_web::web;
use diesel::{ExpressionMethods, QueryResult, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::{db::PgPool, schema::documents};

use super::req::NewDocumentReq;

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
}
