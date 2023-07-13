use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct CreateReferralReq {
    pub user_id: String,
    pub friend_id: String,
}
