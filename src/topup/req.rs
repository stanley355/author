use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TopupPayasyougoReq {
    pub user_id: String,
    pub topup_amount: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TopupPremiumReq {
    pub user_id: String,
    pub duration: TopupPremiumDuration,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TopupPaidReq {
    pub id: String,
}

impl TopupPaidReq {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TopupPremiumDuration {
    Monthly,
    Quarterly,
    HalfYearly,
}

impl fmt::Display for TopupPremiumDuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TopupPremiumDuration::Monthly => write!(f, "monthly"),
            TopupPremiumDuration::Quarterly => write!(f, "quarterly"),
            TopupPremiumDuration::HalfYearly => write!(f, "half_yearly"),
        }
    }
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
