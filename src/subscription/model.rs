use crate::{
    db::PgPool,
    schema::subscriptions,
    topup::{
        model::TopUp,
        req::{TopupPremiumDuration, TopupPremiumReq},
    },
};
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
    pub is_paylater: bool,
}

impl Subscription {
    fn calc_end_timestamp(duration_type: &TopupPremiumDuration) -> NaiveDateTime {
        let days = match duration_type {
            &TopupPremiumDuration::Monthly => 30,
            &TopupPremiumDuration::Quarterly => 90,
            &TopupPremiumDuration::HalfYearly => 180,
        };

        let current_time = Utc::now();
        let end_time = current_time
            .checked_add_signed(Duration::days(days))
            .unwrap();
        return end_time.naive_utc();
    }

    pub fn new(
        pool: &web::Data<PgPool>,
        body: &web::Json<TopupPremiumReq>,
        topup: &TopUp,
    ) -> QueryResult<Subscription> {
        let conn = pool.get().unwrap();

        let end_timestamp = Self::calc_end_timestamp(&body.duration);

        let data = (
            (subscriptions::topup_id.eq(&topup.id)),
            (subscriptions::user_id.eq(&topup.user_id)),
            (subscriptions::end_at.eq(end_timestamp)),
            (subscriptions::duration_type.eq(body.duration.to_string())),
        );

        diesel::insert_into(subscriptions::table)
            .values(data)
            .get_result(&conn)
    }

    pub fn update_paid_subscription(
        pool: &web::Data<PgPool>,
        topup_id: &uuid::Uuid,
    ) -> QueryResult<Subscription> {
        let conn = pool.get().unwrap();

        diesel::update(subscriptions::table)
            .filter(subscriptions::topup_id.eq(topup_id))
            .set(subscriptions::paid.eq(true))
            .get_result(&conn)
    }

    // select * from subscriptions where paid=true order by created_at DESC limit 1;
    pub fn find_active_subscription(
        pool: &web::Data<PgPool>,
        user_id: &str,
    ) -> QueryResult<Subscription> {
        let conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(user_id).unwrap();

        subscriptions::table
            .filter(
                subscriptions::user_id.eq(uuid).and(
                    subscriptions::paid
                        .eq(true)
                        .and(subscriptions::end_at.gt(diesel::dsl::sql("now()"))),
                ),
            )
            .order_by(subscriptions::created_at.desc())
            .get_result::<Subscription>(&conn)
    }
}
