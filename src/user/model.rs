use actix_web::web;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use jsonwebtokens::{encode, Algorithm, AlgorithmID};
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::req::{GmailLoginReq, NoPasswordUser};
use crate::db::PgPool;
use crate::schema::users;
use crate::util::password::generate_random_password;

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub fullname: String,
    pub email: String,
    pub password: String,
    pub phone_number: Option<String>,
}

impl User {
    pub fn find_by_email(pool: &web::Data<PgPool>, email: &str) -> QueryResult<User> {
        let conn = pool.get().unwrap();
        users::table
            .filter(users::email.eq(email))
            .get_result::<User>(&conn)
    }

    pub fn add(pool: &web::Data<PgPool>, body: web::Json<GmailLoginReq>) -> QueryResult<User> {
        let conn = pool.get().unwrap();
        let password = generate_random_password();
        let data = (
            (users::fullname.eq(&body.fullname)),
            (users::email.eq(&body.email)),
            (users::password.eq(password)),
        );

        diesel::insert_into(users::table)
            .values(data)
            .get_result(&conn)
    }

    pub fn remove_password_field(user: User) -> NoPasswordUser {
        NoPasswordUser {
            id: user.id,
            fullname: user.fullname,
            email: user.email,
            phone_number: user.phone_number,
        }
    }

    pub fn create_login_token(user: User) -> String {
        let token_payload = Self::remove_password_field(user);
        let alg = Algorithm::new_hmac(AlgorithmID::HS256, "secret").unwrap();
        let header = json!({ "alg": alg.name() });
        let body = json!(token_payload);
        encode(&header, &body, &alg).unwrap()
    }
}
