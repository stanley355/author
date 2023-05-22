use actix_web::web;
use diesel::{Queryable, QueryResult, ExpressionMethods, RunQueryDsl};
use serde::{Deserialize, Serialize};
use crate::{db::PgPool, schema::checkbots};
use super::req::NewCheckbotReq;

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Checkbot {
  pub id: i32,
  pub user_id: uuid::Uuid,
  pub created_at: chrono::NaiveDateTime,
  pub prompt_token: i32,
  pub completion_token: i32,
  pub prompt_text: String,
  pub completion_text: String,
}

impl Checkbot {
  pub fn new(pool: &web::Data<PgPool>, body: web::Json<NewCheckbotReq>) -> QueryResult<Checkbot> {
    let conn = pool.get().unwrap();
    let uuid = uuid::Uuid::parse_str(&body.user_id).unwrap();
    let data = (
        (checkbots::user_id.eq(uuid)),
        (checkbots::prompt_token.eq(&body.prompt_token)),
        (checkbots::completion_token.eq(&body.completion_token)),
        (checkbots::prompt_text.eq(&body.prompt_text)),
        (checkbots::completion_text.eq(&body.completion_text))
    );

    diesel::insert_into(checkbots::table)
        .values(data)
        .get_result(&conn)
  }
}

