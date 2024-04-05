use actix_web::{web, HttpResponse};
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::req::GmailLoginReq;
use super::res::{GetAccountRes, UserLoginRes};
use crate::db::PgPool;
use crate::schema::users;
use crate::student::model::Student;
use crate::subscription::model::Subscription;
use crate::topup::model::TopUp;
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

    pub fn fetch_account_page_data(pool: &web::Data<PgPool>, user_id: &str) -> GetAccountRes {
        let user_query = Self::find_by_id(pool, user_id);
        let active_student_discount_query = Student::find_active_discount(pool, user_id);
        let active_subscription_query = Subscription::find_active_subscription(pool, user_id);
        let topups_query = TopUp::find_user_topups(pool, user_id);

        match (
            user_query,
            active_student_discount_query,
            active_subscription_query,
            topups_query,
        ) {
            (Ok(user), Ok(active_student_discount), Ok(active_subscription), Ok(topups)) => {
                GetAccountRes::new(
                    Some(user.remove_password_field()),
                    Some(active_student_discount),
                    Some(active_subscription),
                    Some(topups),
                )
            }
            (Ok(user), Ok(active_student_discount), Ok(active_subscription), _) => {
                GetAccountRes::new(
                    Some(user.remove_password_field()),
                    Some(active_student_discount),
                    Some(active_subscription),
                    None,
                )
            }
            (Ok(user), Ok(active_student_discount), _, Ok(topups)) => GetAccountRes::new(
                Some(user.remove_password_field()),
                Some(active_student_discount),
                None,
                Some(topups),
            ),
            (Ok(user), _, Ok(active_subscription), Ok(topups)) => GetAccountRes::new(
                Some(user.remove_password_field()),
                None,
                Some(active_subscription),
                Some(topups),
            ),
            (Ok(user), Ok(active_student_discount), _, _) => GetAccountRes::new(
                Some(user.remove_password_field()),
                Some(active_student_discount),
                None,
                None,
            ),

            (Ok(user), _, Ok(active_subscription), _) => GetAccountRes::new(
                Some(user.remove_password_field()),
                None,
                Some(active_subscription),
                None,
            ),

            (Ok(user), _, _, Ok(topups)) => {
                GetAccountRes::new(Some(user.remove_password_field()), None, None, Some(topups))
            }
            (Ok(user), _, _, _) => {
                GetAccountRes::new(Some(user.remove_password_field()), None, None, None)
            }
            (_, _, _, _) => GetAccountRes::new(None, None, None, None),
        }
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
        topup_id: uuid::Uuid,
        topup_amount: f64,
    ) -> QueryResult<User> {
        let conn = &pool.get().unwrap();

        diesel::update(users::table)
            .filter(users::id.eq(topup_id))
            .set(users::dsl::balance.eq(users::dsl::balance + topup_amount))
            .get_result(conn)
    }

    pub fn reduce_balance(
        pool: &web::Data<PgPool>,
        user_id: uuid::Uuid,
        reduce_amount: f64,
    ) -> QueryResult<User> {
        let conn = &pool.get().unwrap();

        diesel::update(users::table)
            .filter(users::id.eq(user_id))
            .set(users::dsl::balance.eq(users::dsl::balance - reduce_amount))
            .get_result(conn)
    }
}
