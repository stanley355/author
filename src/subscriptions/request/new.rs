use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub(crate) enum SubscriptionDuration {
    Monthly,
    Quarterly,
    HalfYearly,
}

impl fmt::Display for SubscriptionDuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct NewSubscriptionRequest {
    pub user_id: String,
    pub duration: SubscriptionDuration,
}
