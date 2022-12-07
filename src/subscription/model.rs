use super::req::{CreateSubscriptionPayload, ViewSubscriptionPayload};
use crate::db::PgPool;
use crate::schema::subscriptions;

use actix_web::web;
use chrono::Duration;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
use serde::{Deserialize, Serialize};
use std::ops::Add;

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Subscription {
    id: i32,
    user_id: uuid::Uuid,
    channels_id: i32,
    created_at: chrono::NaiveDateTime,
    expired_at: Option<chrono::NaiveDateTime>,
    duration: i32,
}

impl Subscription {
    pub fn create(
        pool: web::Data<PgPool>,
        body: web::Json<CreateSubscriptionPayload>,
    ) -> QueryResult<Subscription> {
        let conn = &pool.get().unwrap();

        let user_uuid = uuid::Uuid::parse_str(&body.user_id).unwrap();

        let current_time = chrono::Utc::now().naive_utc();
        let exp_date = Self::calculate_expired_time(current_time, body.duration.clone());

        let data = (
            (subscriptions::user_id.eq(user_uuid)),
            (subscriptions::channels_id.eq(&body.channels_id)),
            (subscriptions::expired_at.eq(&exp_date)),
            (subscriptions::duration.eq(&body.duration)),
        );

        diesel::insert_into(subscriptions::table)
            .values(data)
            .get_result::<Subscription>(conn)
    }

    pub fn check_subscriptions(
        pool: web::Data<PgPool>,
        query: web::Query<ViewSubscriptionPayload>,
    ) -> QueryResult<Vec<Subscription>> {
        let conn = &pool.get().unwrap();

        let user_uuid = uuid::Uuid::parse_str(&query.user_id).unwrap();

        match query.channels_id {
            Some(channel_id) => subscriptions::table
                .filter(
                    subscriptions::user_id
                        .eq(user_uuid)
                        .and(subscriptions::channels_id.eq(channel_id)),
                )
                .get_results::<Subscription>(conn),
            None => subscriptions::table
                .filter(subscriptions::user_id.eq(user_uuid))
                .get_results::<Subscription>(conn),
        }
    }

    pub fn calculate_expired_time(
        start_time: chrono::NaiveDateTime,
        month_duration: i32,
    ) -> chrono::NaiveDateTime {
        let duration_in_weeks = 4 * month_duration as i64;
        start_time.add(Duration::weeks(duration_in_weeks))
    }
}
