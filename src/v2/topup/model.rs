use actix_web::web;
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use serde::Serialize;

use crate::{db::PgPool, schema::topups};

use super::request::TopupPayasyougoRequestBody;

#[derive(Debug, Serialize, Queryable)]
pub struct TopUp {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub topup_amount: f64,
    pub paid: bool,
    pub topup_type: String,
}

impl TopUp {
    pub fn get_recently_paid(pool: &web::Data<PgPool>, user_id: &str) -> QueryResult<Vec<TopUp>> {
        let mut conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&user_id).unwrap();
        topups::table
            .filter(topups::user_id.eq(uuid).and(topups::paid.eq(true)))
            .order_by(topups::created_at.desc())
            .limit(5)
            .get_results::<TopUp>(&mut conn)
    }

    pub fn new_payasyougo(
        pool: &web::Data<PgPool>,
        body: &web::Json<TopupPayasyougoRequestBody>,
    ) -> QueryResult<TopUp> {
        let mut conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&body.user_id).unwrap();
        
        let data = (
            (topups::user_id.eq(uuid)),
            (topups::topup_amount.eq(&body.amount)),
            // default topup_type is topup
        );

        diesel::insert_into(topups::table)
            .values(data)
            .get_result(&mut conn)
    }
}
