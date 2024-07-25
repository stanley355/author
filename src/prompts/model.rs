use diesel::Queryable;
use serde::Serialize;

use super::payment::PromptPayment;
use super::request::PromptType;

#[derive(Queryable, Debug, Clone, Serialize)]
pub(super) struct Prompt {
    pub id: i32,
    pub user_id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub prompt_token: i32,
    pub completion_token: i32,
    pub prompt_text: String,
    pub completion_text: String,
    pub total_token: i32,
    pub total_cost: f64,
    pub instruction: String,
    pub prompt_type: Option<String>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl Prompt {
   pub(super) fn check_payment(user_id: &uuid::Uuid, prompt_type: &PromptType) -> PromptPayment {

   }
}