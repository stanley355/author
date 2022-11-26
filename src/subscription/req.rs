use diesel::Queryable;
use serde::{Deserialize, Serialize};

use super::enums::Merchant;

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct CreateSubscriptionPayload {
    pub user_id: String,
    pub channels_id: i32,
    pub channels_slug: String,
    pub duration: i32,
    pub invoice_id: String,
    pub channels_name: String,
    pub merchant: Merchant
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct ViewSubscriptionPayload {
    pub user_id: String,
    pub channels_id: Option<i32>,
    pub invoice_id: Option<String>,
}


#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct UpdateSubscriptionChannelPayload {
    pub channels_id: i32,
    pub channels_name: String,
    pub channels_slug: String,
}