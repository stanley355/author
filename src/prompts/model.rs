use actix_web::web;
use diesel::{
    dsl, BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
use serde::Serialize;

use super::payment::PromptPayment;
use super::request::PromptType;
use crate::db::PgPool;
use crate::schema::prompts;
use crate::students::Student;
use crate::subscriptions::Subscription;

#[derive(Queryable, Debug, Clone, Serialize)]
pub(super) struct Prompt {
    pub id: i32,
    pub user_id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub prompt_token: i32,
    pub completion_token: i32,
    pub prompt_text: String,
    pub completion_text: String,
    pub total_token: i32,
    pub total_cost: f64,
    pub instruction: String,
    pub prompt_type: Option<String>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl Prompt {
    pub(super) fn check_payment(
        pool: &web::Data<PgPool>,
        user_id: &uuid::Uuid,
        prompt_type: &PromptType,
    ) -> PromptPayment {
        if let Ok(_) = Student::find_user_last_active_application(pool, user_id) {
            return PromptPayment::Student;
        }

        if let Ok(_) = Subscription::find_active(pool, user_id) {
            return PromptPayment::Subscription;
        }

        if let Ok(count) = Self::count_user_monthly_prompt(pool, user_id, prompt_type) {
            if count <= 5 {
                return PromptPayment::MonthlyQuota;
            }
        }

        PromptPayment::PaymentRequired
    }

    pub fn count_user_monthly_prompt(
        pool: &web::Data<PgPool>,
        user_id: &uuid::Uuid,
        prompt_type: &PromptType,
    ) -> QueryResult<i64> {
        let mut conn = pool.get().unwrap();

        prompts::table
            .filter(
                prompts::user_id
                    .eq(user_id)
                    .and(prompts::prompt_type.eq(prompt_type.to_string()))
                    .and(
                        prompts::created_at
                            .between(dsl::sql("date_trunc('month', now())"), dsl::sql("now()")),
                    ),
            )
            .count()
            .get_result(&mut conn)
    }
}
