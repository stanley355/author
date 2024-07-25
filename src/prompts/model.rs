use diesel::Queryable;
use serde::Serialize;

#[derive(Queryable, Debug, Clone, Serialize)]
pub(crate) struct Prompt {
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
  //  pub fn check_payment() 
}