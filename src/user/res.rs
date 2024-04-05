use serde::{Deserialize, Serialize};

use crate::{student::model::Student, subscription::model::Subscription, topup::model::TopUp};

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
    pub active_student_discount: Option<Student>,
    pub active_subscription: Option<Subscription>,
    pub topups: Option<Vec<TopUp>>,
}

impl GetAccountRes {
    pub fn new(
        user: Option<UserWithoutPassword>,
        active_student_discount: Option<Student>,
        active_subscription: Option<Subscription>,
        topups: Option<Vec<TopUp>>,
    ) -> Self {
        GetAccountRes {
            user,
            active_student_discount,
            active_subscription,
            topups,
        }
    }
}
