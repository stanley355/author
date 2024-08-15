use super::model::User;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub (super) struct UserJwtPayload {
    id: uuid::Uuid,
    fullname: String,
    email: String,
    phone_number: Option<String>,
}

impl UserJwtPayload {
    pub(super) fn new(user: &User) -> Self {
        Self {
            id: user.id,
            fullname: user.fullname.clone(),
            email: user.email.clone(),
            phone_number: user.phone_number.clone()
        }
    }

    pub(super) fn encode(self) -> String {
        let header = Header::new(Algorithm::HS256);
        let key = EncodingKey::from_secret("secret".as_ref());
        let encode_result = encode(&header, &self, &key);

        match encode_result {
            Ok(token) => token,
            Err(_) => "".to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub(super) struct UserJwt {
    token: String,
}

impl UserJwt {
    pub(super) fn new(user: &User) -> Self {
        Self {
            token: UserJwtPayload::new(user).encode(),
        }
    }
}
