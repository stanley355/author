use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FindActiveSubscriptionReq {
    pub user_id: String,
}
