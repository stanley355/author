use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct CreateSubscriptionPayload {
    pub user_id: String,
    pub channels_id: i32,
    pub channels_slug: String,
    pub duration: i32,
    pub invoice_id: String,
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct ViewSubscriptionQuery {
    pub user_id: String,
    pub channels_id: i32,
}