use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct TopUp {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub topup_amount: f64,
    pub paid: bool,
    pub topup_type: String,
}

impl TopUp {
    
}