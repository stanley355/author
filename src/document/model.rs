use actix_web::web;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
use serde::{Deserialize, Serialize};

use crate::{db::PgPool, schema::referral};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Document {
    pub id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub user_id: uuid::Uuid,
    pub name: String,
    pub doc_type: String,
}
