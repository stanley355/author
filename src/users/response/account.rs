use super::base::UsersBaseResponse;
use crate::students::Student;
use crate::subscriptions::Subscription;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub(super) struct UsersAccountResponse {
    user: UsersBaseResponse,
    student: Option<Student>,
    subscription: Option<Subscription>,
}

