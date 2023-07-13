use actix_web::web;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
use serde::{Deserialize, Serialize};

use crate::{db::PgPool, schema::referral};

use super::req::CreateReferralReq;

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Referral {
    pub id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub user_id: uuid::Uuid,
    pub friend_id: uuid::Uuid,
}

impl Referral {
    pub fn new(pool: &web::Data<PgPool>, body: &CreateReferralReq) -> QueryResult<Referral> {
        let conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&body.user_id).unwrap();
        let friend_uuid = uuid::Uuid::parse_str(&body.friend_id).unwrap();
        let data = (
            (referral::user_id.eq(uuid)),
            (referral::friend_id.eq(friend_uuid)),
        );

        diesel::insert_into(referral::table)
            .values(data)
            .get_result(&conn)
    }

    pub fn find(pool: &web::Data<PgPool>, body: &CreateReferralReq) -> QueryResult<Referral> {
        let conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&body.user_id).unwrap();
        let friend_uuid = uuid::Uuid::parse_str(&body.friend_id).unwrap();
        referral::table
            .filter(
                referral::user_id
                    .eq(uuid)
                    .and(referral::friend_id.eq(friend_uuid)),
            )
            .get_result::<Referral>(&conn)
    }
}
