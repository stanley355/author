use super::req::{LoginReq, UpdateUserReq};
use super::res::LoginTokenRes;
use crate::db::PgPool;
use crate::schema::users;

use actix_web::{web, HttpResponse};
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use jsonwebtokens as jwt;
use jwt::{encode, Algorithm, AlgorithmID};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub fullname: String,
    pub email: String,
    pub password: Option<String>,
    pub phone_number: Option<String>,
    pub has_channel: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserInsensitiveData {
    pub id: uuid::Uuid,
    pub fullname: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub has_channel: bool,
}

impl User {
    pub fn check_user(pool: web::Data<PgPool>, email: String) -> QueryResult<User> {
        let conn = &pool.get().unwrap();
        users::table.filter(users::email.eq(email)).get_result(conn)
    }

    pub fn hash_user_data(data: UserInsensitiveData) -> String {
        let alg = Algorithm::new_hmac(AlgorithmID::HS256, "secret").unwrap();
        let header = json!({ "alg": alg.name() });
        let body = json!(data);
        encode(&header, &body, &alg).unwrap()
    }

    pub fn create(pool: web::Data<PgPool>, body: web::Json<LoginReq>) -> QueryResult<User> {
        let conn = &pool.get().unwrap();
        let data = (
            (users::fullname.eq(&body.fullname)),
            (users::email.eq(&body.email)),
        );

        diesel::insert_into(users::table)
            .values(data)
            .get_result(conn)
    }

    pub fn remove_sensitive_data(user: User) -> UserInsensitiveData {
        UserInsensitiveData {
            id: user.id,
            fullname: user.fullname,
            email: user.email,
            phone_number: user.phone_number,
            has_channel: user.has_channel,
        }
    }

    pub fn send_token_response(user: User) -> HttpResponse {
        let insensitive_data = Self::remove_sensitive_data(user);
        let token = Self::hash_user_data(insensitive_data);
        let res = LoginTokenRes::new(token);
        HttpResponse::Ok().json(res)
    }

    pub fn update(pool: web::Data<PgPool>, body: web::Json<UpdateUserReq>) -> QueryResult<User> {
        let conn = &pool.get().unwrap();
        diesel::update(users::table)
            .filter(
                users::fullname
                    .eq(&body.fullname)
                    .and(users::email.eq(&body.email)),
            )
            .set(users::has_channel.eq(&body.has_channel.unwrap()))
            .get_result::<User>(conn)
    }
}
