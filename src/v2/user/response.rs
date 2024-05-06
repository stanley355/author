use diesel::QueryResult;
use serde::Serialize;

use super::{model::User, user_insensitive::UserInsensitive};
use crate::v2::{student::model::Student, subscription::model::Subscription, topup::model::TopUp};

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    token: String,
}

impl LoginResponse {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}

#[derive(Debug, Serialize)]
pub struct AccountPageDataResponse {
    pub user: Option<UserInsensitive>,
    pub active_student_discount: Option<Student>,
    pub active_subscription: Option<Subscription>,
    pub topups: Vec<TopUp>,
}

impl AccountPageDataResponse {
    pub fn new(user_result: QueryResult<User>) -> Self {
        Self {
            user: match user_result {
                Ok(user) => Some(UserInsensitive::new(user)),
                Err(_) => None,
            },
            active_student_discount: None,
            active_subscription: None,
            topups: Vec::new(),
        }
    }
}
