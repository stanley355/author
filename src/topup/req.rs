use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct TopUpReq {
    pub user_id: String,
    pub topup_amount: f64,
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct DokuNotifTransaction{
    pub status: String,
    pub date: String,
    pub original_request_id: String,
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct DokuNotifOrder {
    pub invoice_number: String,
    pub amount: i32,
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct DokuNotifReq {
    pub transaction: DokuNotifTransaction,
    pub order: DokuNotifOrder,
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct PaypalNotifReq{
    pub topup_id: String,
    pub amount: i32,
}
