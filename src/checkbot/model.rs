use actix_web::web;
use diesel::{Queryable, QueryResult, ExpressionMethods, RunQueryDsl, sql_types::Integer};
use serde::{Deserialize, Serialize};
use crate::{db::PgPool, schema::checkbots};
use super::req::NewCheckbotReq;

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Checkbot {
  pub id: i32,
  pub user_id: uuid::Uuid,
  pub created_at: chrono::NaiveDateTime,
  pub source_text_token: i32,
  pub checkbot_text_token: i32,
  pub source_text: String,
  pub checkbot_text: String,
}

impl Checkbot {
  pub fn new(pool: &web::Data<PgPool>, body: web::Json<NewCheckbotReq>) -> QueryResult<Checkbot> {
    let conn = pool.get().unwrap();
    let uuid = uuid::Uuid::parse_str(&body.user_id).unwrap();
    let data = (
        (checkbots::user_id.eq(uuid)),
        (checkbots::source_text_token.eq(1)),
        (checkbots::checkbot_text_token.eq(1)),
        (checkbots::source_text.eq(&body.source_text)),
        (checkbots::source_text.eq(&body.checkbot_text))
    );

    diesel::insert_into(checkbots::table)
        .values(data)
        .get_result(&conn)
  }
}

