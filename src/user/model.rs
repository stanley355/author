use actix_web::{web, HttpResponse};
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::req::{GmailLoginReq, IncreaseBalanceReq, ReduceBalanceReq};
use super::res::UserLoginRes;
use crate::db::PgPool;
use crate::schema::users;
use crate::util::password::Password;
use crate::util::web_response::WebErrorResponse;

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub fullname: String,
    pub email: String,
    pub password: String,
    pub phone_number: Option<String>,
    pub balance: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserWithoutPassword {
    pub id: uuid::Uuid,
    pub fullname: String,
    pub email: String,
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

    pub fn find_by_id(pool: &web::Data<PgPool>, id: &str) -> QueryResult<User> {
        let conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(id).unwrap();
        users::table
            .filter(users::id.eq(uuid))
            .get_result::<User>(&conn)
    }

    pub fn add_from_gmail(
        pool: &web::Data<PgPool>,
        body: web::Json<GmailLoginReq>,
    ) -> QueryResult<User> {
        let conn = pool.get().unwrap();
        let password = Password::generate_random_hash();
        let data = (
            (users::fullname.eq(&body.fullname)),
            (users::email.eq(&body.email)),
            (users::password.eq(password)),
        );

        diesel::insert_into(users::table)
            .values(data)
            .get_result(&conn)
    }

    pub fn remove_password_field(&self) -> UserWithoutPassword {
        UserWithoutPassword {
            id: self.id.clone(),
            fullname: self.fullname.clone(),
            email: self.email.clone(),
            phone_number: self.phone_number.clone(),
            balance: self.balance,
        }
    }

    pub fn create_token(&self) -> String {
        let header = Header::new(Algorithm::HS256);
        let token_data = self.remove_password_field();
        let body = json!(token_data);
        encode(&header, &body, &EncodingKey::from_secret("secret".as_ref())).unwrap()
    }

    pub fn register_user(pool: &web::Data<PgPool>, body: web::Json<GmailLoginReq>) -> HttpResponse {
        let add_result = User::add_from_gmail(&pool, body);

        match add_result {
            Ok(user) => {
                let token = user.create_token();
                HttpResponse::Ok().json(UserLoginRes { token })
            }
            Err(err) => {
                let err_res = WebErrorResponse::server_error(err, "Server Error, please try again");
                HttpResponse::InternalServerError().json(err_res)
            }
        }
    }

    pub fn increase_balance(
        pool: &web::Data<PgPool>,
        body: &IncreaseBalanceReq,
    ) -> QueryResult<User> {
        let conn = &pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&body.user_id).unwrap();

        diesel::update(users::table)
            .filter(users::id.eq(uuid))
            .set(users::dsl::balance.eq(users::dsl::balance + body.increase_amount))
            .get_result(conn)
    }

    pub fn reduce_balance(pool: &web::Data<PgPool>, body: &ReduceBalanceReq) -> QueryResult<User> {
        let conn = &pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&body.user_id).unwrap();

        diesel::update(users::table)
            .filter(users::id.eq(uuid))
            .set(users::dsl::balance.eq(users::dsl::balance - body.reduce_amount))
            .get_result(conn)
    }
}
