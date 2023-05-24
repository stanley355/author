use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct BalanceLogReq {
    pub user_id: String,
    pub prev_balance: f64,
    pub increase_amount: f64,
    pub decrease_amount: f64,
    pub final_balance: f64,
}