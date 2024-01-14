use diesel::Queryable;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct FindActiveSubscriptionReq {
    pub user_id: String,
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct NewSubscriptionReq {
    pub topup_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub duration_type: DurationType,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DurationType {
    Biweekly,
    Monthly,
    Quarterly,
    HalfYearly,
    Yearly,
}

impl fmt::Display for DurationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DurationType::Biweekly=> write!(f, "biweekly"),
            DurationType::Monthly => write!(f, "monthly"),
            DurationType::Quarterly => write!(f, "quarterly"),
            DurationType::HalfYearly => write!(f, "half_yearly"),
            DurationType::Yearly => write!(f, "yearly"),
        }
    }
}