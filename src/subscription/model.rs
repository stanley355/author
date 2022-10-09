use crate::db::PgPool;
use crate::schema::subscriptions;
use actix_web::web;
use diesel::{ExpressionMethods, QueryResult, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use super::req::CreateSubscriptionPayload;

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
        );

        diesel::insert_into(subscriptions::table)
            .values(data)
            .get_result::<Subscription>(conn)
    }
}