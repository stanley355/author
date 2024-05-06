use actix_web::web;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

use super::account_page::AccountPageDataResponse;
use super::request::LoginGmailRequestBody;
use crate::v2::prompt::model::Prompt;
use crate::v2::prompt::prompt_payment::PromptPayment;
use crate::v2::prompt::request::PromptType;
use crate::v2::student::model::Student;
use crate::v2::subscription::model::Subscription;
use crate::{db::PgPool, schema::users};

#[derive(Queryable, Debug, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub fullname: String,
    pub email: String,
    pub password: String,
    pub phone_number: Option<String>,
    pub balance: f64,
}

impl User {
    pub fn find(pool: &web::Data<PgPool>, user_id: &str) -> QueryResult<User> {
        let mut conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(user_id).unwrap();
        users::table
            .filter(users::id.eq(uuid))
            .get_result::<User>(&mut conn)
    }

    pub fn find_by_email(pool: &web::Data<PgPool>, email: &str) -> QueryResult<User> {
        let mut conn = pool.get().unwrap();
        users::table
            .filter(users::email.eq(email))
            .get_result::<User>(&mut conn)
    }

    pub fn check_email_valid(email: &str) -> bool {
        let re = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        re.is_match(email)
    }

    pub fn insert_one_by_gmail(
        pool: &web::Data<PgPool>,
        body: &web::Json<LoginGmailRequestBody>,
    ) -> QueryResult<User> {
        let mut conn = pool.get().unwrap();
        let data = (
            (users::fullname.eq(&body.fullname)),
            (users::email.eq(&body.email)),
            (users::password.eq("")),
        );

        diesel::insert_into(users::table)
            .values(data)
            .get_result(&mut conn)
    }

    pub fn check_prompt_payment(
        pool: &web::Data<PgPool>,
        user_id: &str,
        prompt_type: &PromptType,
    ) -> PromptPayment {
        if let Ok(_) = Student::find_free_discount(pool, user_id) {
            return PromptPayment::Student;
        }

        if let Ok(_) = Subscription::find_active(pool, user_id) {
            return PromptPayment::Subscription;
        }

        if let Ok(user) = Self::find(pool, user_id) {
            if user.balance > 0.0 {
                return PromptPayment::Balance;
            };
        }

        if let Ok(count) = Prompt::count_user_monthly_count(pool, user_id, prompt_type) {
            return match count < 5 {
                true => PromptPayment::MonthlyQuota,
                false => PromptPayment::PaymentRequired,
            };
        }

        PromptPayment::PaymentRequired
    }

    pub fn reduce_balance(
        pool: &web::Data<PgPool>,
        user_id: &str,
        reduce_amount: &f64,
    ) -> QueryResult<User> {
        let mut conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(user_id).unwrap();

        diesel::update(users::table)
            .filter(users::id.eq(uuid))
            .set(users::dsl::balance.eq(users::dsl::balance - reduce_amount))
            .get_result(&mut conn)
    }

    pub fn get_account_page_data(
        pool: &web::Data<PgPool>,
        user_id: &str,
    ) -> AccountPageDataResponse {
        let user_result = Self::find(pool, user_id);
        let student_result = Student::find_active_discount(pool, user_id);
        let subscription_result = Subscription::find_active(pool, user_id);

        let account_page_data =
            AccountPageDataResponse::new(user_result, student_result, subscription_result);
        return account_page_data;
    }
}
