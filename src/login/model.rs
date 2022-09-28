use super::req::LoginReq;
use crate::db::PgPool;
use crate::schema::users;
use actix_web::web;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct LoginUser {
    pub id: uuid::Uuid,
    pub fullname: String,
    pub email: String,
    pub password: Option<String>,
    pub phone_number: Option<String>,
}

impl LoginUser {
    pub fn check_user(pool: web::Data<PgPool>, email: String) -> QueryResult<LoginUser> {
        let conn = &pool.get().unwrap();
        users::table.filter(users::email.eq(email)).get_result(conn)
    }

    pub fn hash_user_data(data: LoginUser) -> String {
        let data_str = format!("{:?}", data);
        let mut hasher = Sha256::new();
        hasher.update(data_str);
        format!("{:x}", hasher.finalize())
    }
    
    pub fn add(pool: web::Data<PgPool>, body: web::Json<LoginReq>) -> QueryResult<LoginUser> {
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
}
