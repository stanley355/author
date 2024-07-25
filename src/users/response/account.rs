use super::base::UsersBaseResponse;
use crate::db::PgPool;
use crate::students::Student;
use crate::subscriptions::Subscription;
use crate::users::model::User;
use actix_web::web;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct UsersAccountResponse {
    user: UsersBaseResponse,
    student: Option<Student>,
    subscription: Option<Subscription>,
    subscriptions: Option<Vec<Subscription>>,
}

impl UsersAccountResponse {
    pub(crate) fn new(pool: &web::Data<PgPool>, user: &User) -> Self {
        let student_result = match Student::find_user_last_active_application(pool, &user.id) {
            Ok(student) => Some(student),
            Err(_) => None,
        };

        let subscription_result = match Subscription::find_active(pool, &user.id) {
            Ok(subscription) => Some(subscription),
            Err(_) => None,
        };

        let subscriptions_result = match Subscription::find_last_five(pool, &user.id) {
            Ok(subscriptions) => Some(subscriptions),
            Err(_) => None,
        };

        Self {
            user: UsersBaseResponse::new(user),
            student: student_result,
            subscription: subscription_result,
            subscriptions: subscriptions_result,
        }
    }
}
