use actix_web::web;
use diesel::{ExpressionMethods, QueryResult, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use super::req::TopUpReq;
use crate::db::PgPool;
use crate::schema::topups;

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct TopUp {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub topup_amount: f64,
    pub paid: bool,
}

impl TopUp {
    pub fn new(pool: &web::Data<PgPool>, body: &TopUpReq) -> QueryResult<TopUp> {
        let conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&body.user_id).unwrap();
        let data = (
            (topups::user_id.eq(uuid)),
            (topups::topup_amount.eq(&body.topup_amount)),
        );

        diesel::insert_into(topups::table)
            .values(data)
            .get_result(&conn)
    }
}
