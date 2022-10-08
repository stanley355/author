use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Subscription {
  id: i32,
  user_id: uuid::Uuid,
  channels_id: i32,
  channels_slug: String,
  created_at: chrono::NaiveDateTime,
  expired_at: Option<chrono::NaiveDateTime>,
  monthly_price: i32,
  total_price: i32,
  status: String
}