use super::model::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserTokenData {
    pub id: uuid::Uuid,
    pub fullname: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub has_channel: bool,
    pub subscriptions: Vec<String>,
}

impl UserTokenData {
    pub fn new(user: User) -> UserTokenData {
        UserTokenData {
            id: user.id,
            fullname: user.fullname,
            email: user.email,
            phone_number: user.phone_number,
            has_channel: user.has_channel,
            subscriptions: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginTokenRes {
    pub token: String,
}

impl LoginTokenRes {
    pub fn new(token: String) -> Self {
        LoginTokenRes { token }
    }
}
