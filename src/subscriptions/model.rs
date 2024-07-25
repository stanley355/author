use super::request::{NewSubscriptionRequest, SubscriptionDuration};
use crate::db::PgPool;
use crate::schema::subscriptions;
use actix_web::web;
use diesel::{ExpressionMethods, QueryResult, Queryable, RunQueryDsl};
use serde::Serialize;

#[derive(Queryable, Debug, Serialize)]
pub struct Subscription {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub start_at: chrono::NaiveDateTime,
    pub end_at: chrono::NaiveDateTime,
    pub duration_type: String,
    pub paid: bool,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub price: f64,
}

impl Subscription {
    pub(super) fn new_insert(
        pool: &web::Data<PgPool>,
        request: &NewSubscriptionRequest,
        is_student: bool,
    ) -> QueryResult<Subscription> {
        let user_id = uuid::Uuid::parse_str(&request.user_id).unwrap();
        let end_at = Self::calc_end_at(&request.duration);
        let price = Self::calc_price(&request.duration, is_student);

        let data = (
            (subscriptions::user_id.eq(user_id)),
            (subscriptions::end_at.eq(end_at)),
            (subscriptions::duration_type.eq(request.duration.to_string())),
            (subscriptions::price.eq(price)),
        );

        let mut conn = pool.get().unwrap();
        diesel::insert_into(subscriptions::table)
            .values(data)
            .get_result(&mut conn)
    }

    fn calc_end_at(duration: &SubscriptionDuration) -> chrono::NaiveDateTime {
        let days = match duration {
            SubscriptionDuration::Monthly => 30,
            SubscriptionDuration::Quarterly => 90,
            SubscriptionDuration::HalfYearly => 180,
        };

        chrono::Utc::now()
            .checked_add_signed(chrono::Duration::days(days))
            .unwrap()
            .naive_utc()
    }

    fn calc_price(duration: &SubscriptionDuration, is_student: bool) -> f64 {
        match is_student {
            true => match duration {
                SubscriptionDuration::Monthly => 12500.0,
                SubscriptionDuration::Quarterly => 30000.0,
                SubscriptionDuration::HalfYearly => 70000.0,
            },
            false => match duration {
                SubscriptionDuration::Monthly => 25000.0,
                SubscriptionDuration::Quarterly => 70000.0,
                SubscriptionDuration::HalfYearly => 150000.0,
            },
        }
    }

    pub fn update_paid(
        pool: &web::Data<PgPool>,
        request_subscription_id: &str,
    ) -> QueryResult<Subscription> {
        let mut conn = pool.get().unwrap();
        let subscription_id = uuid::Uuid::parse_str(request_subscription_id).unwrap();

        diesel::update(subscriptions::table)
            .filter(subscriptions::id.eq(subscription_id))
            .set(subscriptions::paid.eq(true))
            .get_result(&mut conn)
    }
}
