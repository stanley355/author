use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub fullname: String,
    pub email: String,
    pub password: Option<String>,
    pub phone_number: Option<String>,
    pub has_channel: bool,
}

impl User {
    
}
