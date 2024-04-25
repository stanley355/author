use serde::{Deserialize, Serialize};
use actix_web::web;
use crate::db::PgPool;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub fullname: String,
    pub email: String,
    pub password: String,
    pub phone_number: Option<String>,
    pub balance: f32,
}

impl User {
  pub fn has_prompt_quota(pool: &web::Data<PgPool>, user_id: &str) -> bool { 
    

    false
  }
}