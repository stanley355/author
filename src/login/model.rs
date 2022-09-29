use super::req::LoginReq;
use super::res::LoginRes;
use crate::db::PgPool;
use crate::schema::users;

use actix_web::{web, HttpResponse};
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
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
}

impl User {
    pub fn check_user(pool: web::Data<PgPool>, email: String) -> QueryResult<User> {
        let conn = &pool.get().unwrap();
        users::table.filter(users::email.eq(email)).get_result(conn)
    }

    pub fn hash_user_data(data: User) -> String {
        let alg = Algorithm::new_hmac(AlgorithmID::HS256, "secret").unwrap();
        let header = json!({ "alg": alg.name() });
        let body = json!(data);
        encode(&header, &body, &alg).unwrap()
    }

    pub fn add(pool: web::Data<PgPool>, body: web::Json<LoginReq>) -> QueryResult<User> {
        let conn = &pool.get().unwrap();
        let data = (
            (users::fullname.eq(&body.fullname)),
            (users::email.eq(&body.email)),
            (users::password.eq(&body.password)),
            (users::phone_number.eq(&body.phone_number)),
        );

        diesel::insert_into(users::table)
            .values(data)
            .get_result(conn)
    }

    pub fn send_token_response(user: User) -> HttpResponse {
        let token = Self::hash_user_data(user);
        let res = LoginRes::new(token);
        HttpResponse::Ok().json(res)
    }
}
