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
    channels_slug: String,
    created_at: chrono::NaiveDateTime,
    expired_at: Option<chrono::NaiveDateTime>,
    paid: bool,
    duration: i32,
    invoice_id: String,
    channels_name: String,
}

impl Subscription {
    pub fn create(
        pool: web::Data<PgPool>,
        body: web::Json<CreateSubscriptionPayload>,
    ) -> QueryResult<Subscription> {
        let conn = &pool.get().unwrap();

        let user_uuid = uuid::Uuid::parse_str(&body.user_id).unwrap();

        let data = (
            (subscriptions::user_id.eq(user_uuid)),
            (subscriptions::channels_id.eq(&body.channels_id)),
            (subscriptions::channels_slug.eq(&body.channels_slug)),
            (subscriptions::duration.eq(&body.duration)),
            (subscriptions::invoice_id.eq(&body.invoice_id)),
            (subscriptions::channels_name.eq(&body.channels_name))
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
            Some(channel_id) => {
                subscriptions::table
                .filter(
                    subscriptions::user_id
                        .eq(user_uuid)
                        .and(subscriptions::channels_id.eq(channel_id)),
                )
                .get_results::<Subscription>(conn)
            },
            None => {
                subscriptions::table
                .filter(subscriptions::user_id.eq(user_uuid))
                .get_results::<Subscription>(conn)
            }
        }
    }

    pub fn check_subscription(
        pool: web::Data<PgPool>,
        query: web::Query<ViewSubscriptionPayload>,
    ) -> QueryResult<Subscription> {
        let conn = &pool.get().unwrap();

        let user_uuid = uuid::Uuid::parse_str(&query.user_id).unwrap();

        let channel_id = query.channels_id.clone().unwrap();
        let invoice_id = query.invoice_id.clone().unwrap();

        subscriptions::table
                .filter(
                    subscriptions::user_id
                        .eq(user_uuid)
                        .and(subscriptions::channels_id.eq(&channel_id))
                        .and(subscriptions::invoice_id.eq(&invoice_id)),
                )
                .get_result::<Subscription>(conn)
    }

    pub fn update_paid_subscription(
        pool: web::Data<PgPool>,
        body: Subscription,
    ) -> QueryResult<Subscription> {
        let conn = &pool.get().unwrap();

        let expired_date = Self::calculate_expired_time(body.created_at, body.duration);

        let data = (
            (subscriptions::paid.eq(true)),
            (subscriptions::expired_at.eq(&expired_date)),
        );

        diesel::update(subscriptions::table)
            .filter(
                subscriptions::user_id
                    .eq(&body.user_id)
                    .and(subscriptions::channels_id.eq(&body.channels_id))
                    .and(subscriptions::invoice_id.eq(&body.invoice_id)),
            )
            .set(data)
            .get_result(conn)
    }

    pub fn calculate_expired_time(
        exp_time: chrono::NaiveDateTime,
        month_duration: i32,
    ) -> chrono::NaiveDateTime {
        let duration_in_weeks = 4 * month_duration as i64;
        exp_time.add(Duration::weeks(duration_in_weeks))
    }
}
