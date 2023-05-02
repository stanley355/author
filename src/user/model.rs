use actix_web::web;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use serde::{Deserialize, Serialize};

use super::req::GmailLoginReq;
use crate::db::PgPool;
use crate::schema::users;

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
        let data = (
            (users::fullname.eq(&body.fullname)),
            (users::email.eq(&body.email)),
        );
        
        diesel::insert_into(users::table)
            .values(data)
            .get_result(&conn)
    }
}
