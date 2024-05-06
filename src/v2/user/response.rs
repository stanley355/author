use serde::Serialize;

use super::user_insensitive::UserInsensitive;
use crate::v2::{student::model::Student, subscription::model::Subscription};

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    token: String,
}

impl LoginResponse {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}

#[derive(Debug)]
pub struct AccountPageDataResponse {
    pub user: UserInsensitive,
    pub active_student_discount: Option<Student>,
    pub active_subscription: Option<Subscription>,
}
