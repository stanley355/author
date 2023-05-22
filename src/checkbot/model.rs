use actix_web::web;
use diesel::Queryable;
use diesel::sql_types::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Checkbot {
  pub id: u32,
  pub user_id: uuid::Uuid,
  pub created_at: Timestamp,
  pub source_text_token: u32,
  pub checkbot_text_token: u32,
  pub source_text: String,
  pub checkbot_text: String,
}
