use diesel::Queryable;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct NewSubscriptionReq {
    pub user_id: String,
    pub duration_type: DurationType,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DurationType {
    Monthly,
    Quarterly,
    HalfYearly,
    Yearly,
}

impl fmt::Display for DurationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DurationType::Monthly => write!(f, "monthly"),
            DurationType::Quarterly => write!(f, "Quarterly"),
            DurationType::HalfYearly => write!(f, "HalfYearly"),
            DurationType::Yearly => write!(f, "Yearly"),
        }
    }
}
