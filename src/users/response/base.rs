use crate::users::model::User;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub(super) struct UsersBaseResponse {
    id: uuid::Uuid,
    fullname: String,
    email: String,
    phone_number: Option<String>,
}

impl UsersBaseResponse {
    pub(super) fn new(user: &User) -> Self {
        Self {
            id: user.id,
            fullname: user.fullname.clone(),
            email: user.fullname.clone(),
            phone_number: user.phone_number.clone(),
        }
    }
}
