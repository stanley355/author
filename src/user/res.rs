use serde::{Deserialize, Serialize};

use crate::{subscription::model::Subscription, topup::model::TopUp};

use super::model::UserWithoutPassword;

// TODO: Remove this after refac all error
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrorRes {
    pub error: String,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserLoginRes {
    pub token: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetAccountRes {
    pub user: Option<UserWithoutPassword>,
    pub active_subscription: Option<Subscription>,
    pub topups: Option<Vec<TopUp>>,
}

impl GetAccountRes {
    pub fn new(
        user: Option<UserWithoutPassword>,
        active_subscription: Option<Subscription>,
        topups: Option<Vec<TopUp>>,
    ) -> Self {
        GetAccountRes {
            user,
            active_subscription,
            topups,
        }
    }
}
