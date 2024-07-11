use crate::v2::topup::model::TopUp;
use crate::v2::topup::request::{TopupPremiumDuration, TopupPremiumRequestBody};
use crate::{db::PgPool, schema::subscriptions};
use actix_web::web;
use chrono::{Duration, Utc};
use diesel::{
    BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
use serde::Serialize;

#[derive(Queryable, Debug, Serialize)]
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
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl Subscription {
    pub fn find_active(pool: &web::Data<PgPool>, user_id: &str) -> QueryResult<Subscription> {
        let mut conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(user_id).unwrap();

        subscriptions::table
            .filter(
                subscriptions::user_id
                    .eq(uuid)
                    .and(subscriptions::paid.eq(true))
                    .and(subscriptions::end_at.gt(diesel::dsl::sql("now()"))),
            )
            .order_by(subscriptions::created_at.desc())
            .get_result::<Subscription>(&mut conn)
    }

    fn calc_end_timestamp(duration_type: &TopupPremiumDuration) -> chrono::NaiveDateTime {
        let days = match duration_type {
            TopupPremiumDuration::Monthly => 30,
            TopupPremiumDuration::Quarterly => 90,
            TopupPremiumDuration::HalfYearly => 180,
        };

        let end_time = Utc::now().checked_add_signed(Duration::days(days)).unwrap();
        return end_time.naive_utc();
    }

    pub fn insert_from_topup(
        pool: &web::Data<PgPool>,
        body: &web::Json<TopupPremiumRequestBody>,
        topup: &TopUp,
    ) -> QueryResult<Subscription> {
        let mut conn = pool.get().unwrap();

        let end_timestamp = Self::calc_end_timestamp(&body.duration);

        let data = (
            (subscriptions::topup_id.eq(&topup.id)),
            (subscriptions::user_id.eq(&topup.user_id)),
            (subscriptions::end_at.eq(end_timestamp)),
            (subscriptions::duration_type.eq(body.duration.to_string())),
        );

        diesel::insert_into(subscriptions::table)
            .values(data)
            .get_result(&mut conn)
    }

    pub fn update_paid_subscription(
        pool: &web::Data<PgPool>,
        topup_id: &str,
    ) -> QueryResult<Subscription> {
        let mut conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(topup_id).unwrap();

        diesel::update(subscriptions::table)
            .filter(subscriptions::topup_id.eq(uuid))
            .set(subscriptions::paid.eq(true))
            .get_result(&mut conn)
    }
}
