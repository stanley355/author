use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct CreateSubscriptionPayload {
    pub user_id: String,
    pub channels_id: i32,
    pub channels_slug: String,
    pub duration: i32,
    pub channels_name: String,
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct ViewSubscriptionPayload {
    pub user_id: String,
    pub channels_id: Option<i32>,
}

