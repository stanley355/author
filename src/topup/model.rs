use actix_web::web;
use diesel::{ExpressionMethods, QueryResult, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use super::req::{DokuNotifReq, TopUpReq, TopUpSubscriptionReq};
use crate::db::PgPool;
use crate::schema::topups;

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct TopUp {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub topup_amount: f64,
    pub paid: bool,
    pub topup_type: String
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

    pub fn new_subscription(pool: &web::Data<PgPool>, body: &TopUpSubscriptionReq) -> QueryResult<TopUp> {
        let conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&body.user_id).unwrap();
        let data = (
            (topups::user_id.eq(uuid)),
            (topups::topup_amount.eq(&body.topup_amount)),
            (topups::topup_type.eq("subscription".to_string()))
        );

        diesel::insert_into(topups::table)
            .values(data)
            .get_result(&conn)
    }

    pub fn verify_doku_paid_status(
        pool: &web::Data<PgPool>,
        body: &DokuNotifReq,
    ) -> QueryResult<TopUp> {
        let conn = pool.get().unwrap();
        let topup_id = uuid::Uuid::parse_str(&body.transaction.original_request_id).unwrap();

        diesel::update(topups::table)
            .filter(topups::id.eq(topup_id))
            .set(topups::paid.eq(true))
            .get_result(&conn)
    }
}
