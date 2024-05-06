use serde::Serialize;
use diesel::QueryResult;
use super::{model::User, user_insensitive::UserInsensitive};
use crate::v2::{student::model::Student, subscription::model::Subscription, topup::model::TopUp};


#[derive(Debug, Serialize)]
pub struct AccountPageDataResponse {
    pub user: Option<UserInsensitive>,
    pub active_student_discount: Option<Student>,
    pub active_subscription: Option<Subscription>,
    pub topups: Vec<TopUp>,
}

impl AccountPageDataResponse {
    pub fn new(user_result: QueryResult<User>, student_result: QueryResult<Student>, subscription_result: QueryResult<Subscription>) -> Self {
        Self {
            user: match user_result {
                Ok(user) => Some(UserInsensitive::new(user)),
                Err(_) => None,
            },
            active_student_discount: match student_result {
                Ok(student) => Some(student),
                Err(_) => None,
            },
            active_subscription: match subscription_result {
               Ok(subscription) => Some(subscription),
               Err(_) => None 
            },
            topups: Vec::new(),
        }
    }
}