use super::req::{DurationType, NewSubscriptionReq};
use crate::{db::PgPool, schema::subscriptions};
use actix_web::web;
use chrono::{Duration, NaiveDateTime, Utc};
use diesel::{
    BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Subscription {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub topup_id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub start_at: chrono::NaiveDateTime,
    pub end_at: chrono::NaiveDateTime,
    pub duration_type: String,
    pub paid: bool,
}

impl Subscription {
    pub fn new(pool: &web::Data<PgPool>, body: &NewSubscriptionReq) -> QueryResult<Subscription> {
        let conn = pool.get().unwrap();
        let user_id = uuid::Uuid::parse_str(&body.user_id).unwrap();

        let end_timestamp = Self::calc_end_timestamp(&body.duration_type);

        let data = (
            (subscriptions::topup_id.eq(&body.topup_id)),
            (subscriptions::user_id.eq(user_id)),
            (subscriptions::end_at.eq(end_timestamp)),
            (subscriptions::duration_type.eq(body.duration_type.to_string())),
        );

        diesel::insert_into(subscriptions::table)
            .values(data)
            .get_result(&conn)
    }

    fn calc_end_timestamp(duration_type: &DurationType) -> NaiveDateTime {
        let days = match duration_type {
            DurationType::Monthly => 30,
            DurationType::Quarterly => 90,
            DurationType::HalfYearly => 180,
            DurationType::Yearly => 365,
        };

        let current_time = Utc::now();
        let end_time = current_time
            .checked_add_signed(Duration::days(days))
            .unwrap();
        return end_time.naive_utc();
    }
}
