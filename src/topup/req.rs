use crate::subscription::req::DurationType;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct TopUpReq {
    pub user_id: String,
    pub topup_amount: f64,
    pub subscription: Option<DurationType>,
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct DokuNotifTransaction {
    pub status: String,
    pub date: String,
    pub original_request_id: String,
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct DokuNotifReq {
    pub transaction: DokuNotifTransaction,
}
