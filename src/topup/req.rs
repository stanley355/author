use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TopupPayasyougoReq {
    pub user_id: String,
    pub topup_amount: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DokuNotifTransaction {
    pub status: String,
    pub date: String,
    pub original_request_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DokuNotifReq {
    pub transaction: DokuNotifTransaction,
}
