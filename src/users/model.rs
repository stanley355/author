use actix_web::web;
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::{ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl};

use crate::db::PgPool;
use crate::schema::users;
use super::request::UsersLoginGmailRequest;

#[derive(Debug, Queryable)]
pub(super) struct User {
    pub id: uuid::Uuid,
    pub fullname: String,
    pub email: String,
    pub password: String,
    #[allow(dead_code)]
    pub phone_number: Option<String>,
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

    pub(super) fn new_from_login_gmail_insert(
        pool: &web::Data<PgPool>,
        request: &UsersLoginGmailRequest,
    ) -> QueryResult<User> {
        let mut conn = pool.get().unwrap();
        let data = (
            (users::fullname.eq(&request.fullname)),
            (users::email.eq(&request.email)),
            (users::password.eq("")),
        );

        diesel::insert_into(users::table)
            .values(data)
            .get_result(&mut conn)
    }

    pub(super) fn check_password_valid(&self, password: &str) -> bool {
        verify(password, &self.password).unwrap()
    }

    pub(super) fn change_password(
        pool: &web::Data<PgPool>,
        user_id: &uuid::Uuid,
        new_password: &str,
    ) -> QueryResult<User> {
        let hashed_password = hash(new_password, DEFAULT_COST).unwrap();
        let mut conn = pool.get().unwrap();
        diesel::update(users::table)
            .filter(users::id.eq(user_id))
            .set(users::password.eq(hashed_password))
            .get_result(&mut conn)
    }
}
