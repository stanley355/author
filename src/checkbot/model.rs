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
  pub fn new(pool: &web::Data<PgPool>, body: web::Json<NewCheckbotReq>) -> QueryResult<Vec<Checkbot>> {
    let conn = pool.get().unwrap();
    let uuid = uuid::Uuid::parse_str(&body.user_id).unwrap();
    println!("uuid: {}", uuid);
    // let data = (
    //     (checkbots::user_id.eq(uuid)),
    //     (checkbots::source_text_token.eq(1)),
    //     (checkbots::checkbot_text_token.eq(1)),
    //     (checkbots::source_text.eq(&body.source_text)),
    //     (checkbots::source_text.eq(&body.checkbot_text))
    // );

    checkbots::table.get_results(&conn)

    // diesel::insert_into(checkbots::table)
    //     .values(data)
    //     .get_result(&conn)
  }
}

