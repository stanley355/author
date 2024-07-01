use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct TopupPremiumRequestBody {
    pub user_id: String,
    pub duration: TopupPremiumDuration,
}

#[derive(Debug, Deserialize)]
pub enum TopupPremiumDuration {
    Monthly,
    Quarterly,
    HalfYearly,
}

impl fmt::Display for TopupPremiumDuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
