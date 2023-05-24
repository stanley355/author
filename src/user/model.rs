use actix_web::web;
use bcrypt::{hash, DEFAULT_COST};
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::req::{GmailLoginReq, IncreaseBalanceReq, LoginReq, RegisterReq};
use super::res::NoPasswordUser;
use crate::db::PgPool;
use crate::schema::users;
use crate::schema::users::dsl;
use crate::util::password::generate_random_password;

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub fullname: String,
    pub email: String,
    pub password: String,
    pub phone_number: Option<String>,
    pub balance: f64,
}

impl User {
    pub fn find_by_email(pool: &web::Data<PgPool>, email: &str) -> QueryResult<User> {
        let conn = pool.get().unwrap();
        users::table
            .filter(users::email.eq(email))
            .get_result::<User>(&conn)
    }

    pub fn add_from_register(
        pool: &web::Data<PgPool>,
        body: web::Json<RegisterReq>,
    ) -> QueryResult<User> {
        let conn = pool.get().unwrap();
        let password = Self::hash_password(&body.password);
        let data = (
            (users::fullname.eq(&body.fullname)),
            (users::email.eq(&body.email)),
            (users::password.eq(password)),
        );

        diesel::insert_into(users::table)
            .values(data)
            .get_result(&conn)
    }

    pub fn add_from_gmail(
        pool: &web::Data<PgPool>,
        body: web::Json<GmailLoginReq>,
    ) -> QueryResult<User> {
        let conn = pool.get().unwrap();
        let password = generate_random_password();
        let hashed_password = Self::hash_password(&password);
        let data = (
            (users::fullname.eq(&body.fullname)),
            (users::email.eq(&body.email)),
            (users::password.eq(&hashed_password)),
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
            balance: user.balance,
        }
    }

    pub fn update_password(
        pool: &web::Data<PgPool>,
        body: web::Json<LoginReq>,
    ) -> QueryResult<User> {
        let conn = &pool.get().unwrap();
        let password = Self::hash_password(&body.password);

        diesel::update(users::table)
            .filter(users::email.eq(&body.email))
            .set(users::password.eq(password))
            .get_result(conn)
    }

    pub fn hash_password(password: &str) -> String {
        hash(password, DEFAULT_COST).unwrap()
    }

    pub fn create_login_token(user: User) -> String {
        let header = Header::new(Algorithm::HS256);
        let token_payload = Self::remove_password_field(user);
        let body = json!(token_payload);
        encode(&header, &body, &EncodingKey::from_secret("secret".as_ref())).unwrap()
    }

    pub fn increase_balance(
        pool: &web::Data<PgPool>,
        body: &web::Json<IncreaseBalanceReq>,
    ) -> QueryResult<User> {
        let conn = &pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&body.user_id).unwrap();

        diesel::update(users::table)
            .filter(users::id.eq(uuid))
            .set(dsl::balance.eq(dsl::balance + body.increase_amount))
            .get_result(conn)
    }
}
