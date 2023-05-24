use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct TopUpReq {
    pub user_id: String,
    pub topup_amount: f64,
}