use actix_web::web;
use diesel::{ExpressionMethods, QueryResult, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use super::req::BalanceLogReq;
use crate::db::PgPool;
use crate::schema::balance_logs;

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct BalanceLog {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub prev_balance: f64,
    pub increase_amount: f64,
    pub decrease_amount: f64,
    pub final_balance: f64,
}

impl BalanceLog {
    pub fn new(pool: &web::Data<PgPool>, body: &BalanceLogReq) -> QueryResult<BalanceLog> {
        let conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&body.user_id).unwrap();
        let final_balance = &body.prev_balance + &body.increase_amount - &body.decrease_amount;
        let data = (
            (balance_logs::user_id.eq(uuid)),
            (balance_logs::prev_balance.eq(&body.prev_balance)),
            (balance_logs::increase_amount.eq(&body.increase_amount)),
            (balance_logs::decrease_amount.eq(&body.decrease_amount)),
            (balance_logs::final_balance.eq(final_balance)),
        );

        diesel::insert_into(balance_logs::table)
            .values(data)
            .get_result(&conn)
    }
}
