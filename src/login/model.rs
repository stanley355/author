use super::req::LoginReq;
use crate::db::PgPool;
use crate::schema::users;
use actix_web::web;
use diesel::{ExpressionMethods, QueryResult, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct LoginUser {
    pub id: uuid::Uuid,
    pub fullname: String,
    pub email: String,
    pub password: Option<String>,
    pub phone_number: Option<String>,
}

impl LoginUser {
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

    // pub fn check_existence(pool: web::Data<PgPool>) -> QueryResult<bool>
}
