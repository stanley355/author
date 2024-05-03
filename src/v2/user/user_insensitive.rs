use super::model::User;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::Serialize;

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

    pub fn jwt_tokenize(self) -> String {
        let header = Header::new(Algorithm::HS256);
        let key = EncodingKey::from_secret("secret".as_ref());
        let token_result = encode(&header, &self, &key);

        match token_result {
            Ok(token) => token,
            Err(_) => "".to_string(),
        }
    }
}
