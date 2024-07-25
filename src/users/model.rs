use actix_web::web;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl};

use crate::db::PgPool;
use crate::schema::users;

#[derive(Debug, Queryable)]
pub(super) struct User {
    pub id: uuid::Uuid,
    pub fullname: String,
    pub email: String,
    #[allow(dead_code)]
    pub password: String,
    #[allow(dead_code)]
    pub phone_number: Option<String>,
    #[allow(dead_code)]
    pub balance: f64,
    #[allow(dead_code)]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[allow(dead_code)]
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl User {
    pub(super) fn find(pool: &web::Data<PgPool>, user_id: uuid::Uuid) -> QueryResult<User> {
        let mut conn = pool.get().unwrap();
        users::table
            .filter(users::id.eq(user_id))
            .get_result::<User>(&mut conn)
    }

    pub(super) fn find_by_email(pool: &web::Data<PgPool>, email: &str) -> QueryResult<User> {
        let mut conn = pool.get().unwrap();
        users::table
            .filter(users::email.eq(email))
            .get_result::<User>(&mut conn)
    }
}
