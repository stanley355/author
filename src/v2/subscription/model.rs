use crate::{db::PgPool, schema::subscriptions};
use actix_web::web;
use chrono::{Duration, NaiveDateTime, Utc};
use diesel::{
    BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone)]
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
    pub fn find_active(
        pool: &web::Data<PgPool>,
        user_id: &str,
    ) -> QueryResult<Subscription> {
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
}
