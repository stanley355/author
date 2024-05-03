use serde::Serialize;
use super::model::User;

#[derive(Debug, Serialize)]
pub struct UserInsensitive {
    pub id: uuid::Uuid,
    pub fullname: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub balance: f64,
}

impl UserInsensitive {
    pub fn new(user: User) -> Self {
        Self {
            id: user.id,
            fullname: user.fullname,
            email: user.email,
            phone_number: user.phone_number,
            balance: user.balance,
        }
    }
}
