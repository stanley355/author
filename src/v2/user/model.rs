use actix_web::web;
use serde::{Deserialize, Serialize};

use crate::db::PgPool;
use crate::v2::student::model::Student;

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
        if let Ok(student) = Student::find_free_discount(pool, user_id) {
            return true;
        }

        false
    }
}
