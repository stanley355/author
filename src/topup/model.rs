use actix_web::web;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use super::req::{TopupPaidReq, TopupPayasyougoReq, TopupPremiumDuration, TopupPremiumReq};
use crate::db::PgPool;
use crate::schema::topups;

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct TopUp {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub topup_amount: f64,
    pub paid: bool,
    pub topup_type: String,
}

impl TopUp {
    pub fn new_payasyougo(
        pool: &web::Data<PgPool>,
        body: &web::Json<TopupPayasyougoReq>,
    ) -> QueryResult<TopUp> {
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

    pub fn calc_premium_price(duration: &TopupPremiumDuration) -> f64 {
        match duration {
            TopupPremiumDuration::Monthly => 25000.0,
            TopupPremiumDuration::Quarterly => 70000.0,
            TopupPremiumDuration::HalfYearly => 150000.0,
        }
    }

    pub fn new_premium(
        pool: &web::Data<PgPool>,
        body: &web::Json<TopupPremiumReq>,
    ) -> QueryResult<TopUp> {
        let conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&body.user_id).unwrap();
        let price = Self::calc_premium_price(&body.duration);

        let data = (
            (topups::user_id.eq(uuid)),
            (topups::topup_amount.eq(&price)),
            (topups::topup_type.eq("subscription".to_string())),
        );

        diesel::insert_into(topups::table)
            .values(data)
            .get_result(&conn)
    }

    pub fn find_user_topups(pool: &web::Data<PgPool>, user_id: &str) -> QueryResult<Vec<TopUp>> {
        let conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&user_id).unwrap();
        topups::table
            .filter(topups::user_id.eq(uuid))
            .order_by(topups::created_at.desc())
            .limit(5)
            .get_results::<TopUp>(&conn)
    }

    pub fn update_paid_topup(
        pool: &web::Data<PgPool>,
        body: &web::Json<TopupPaidReq>,
    ) -> QueryResult<TopUp> {
        let conn = pool.get().unwrap();
        let topup_id = uuid::Uuid::parse_str(&body.id).unwrap();

        diesel::update(topups::table)
            .filter(topups::id.eq(topup_id))
            .set(topups::paid.eq(true))
            .get_result(&conn)
    }
}
