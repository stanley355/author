use actix_web::web;
use diesel::{ExpressionMethods, QueryResult, Queryable, RunQueryDsl, QueryDsl};
use serde::{Deserialize, Serialize};

use super::req::{DokuNotifReq, TopUpReq};
use crate::db::PgPool;


use crate::schema::{topups, users};

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

    // pub fn update_balance_from_doku_notif(
    //     pool: &web::Data<PgPool>,
    //     body: &DokuNotifReq,
    // ) -> QueryResult<TopUp> {
    //     let conn = pool.get().unwrap();
    //     let topup_id = uuid::Uuid::parse_str(&body.transaction.original_request_id);
        
    //     diesel::update(topups::table)
    //         .filter(topups::id.eq(topup_id))
    //         .left_join(users::table.on(topups::dsl::user_id.eq(users::id)))
    //         .select((topups::id, topups::user_id, users::balance))
    //         .set()
    //         .get_result(conn)
    // }
}
