use super::{req::NewPromptReq, res::NewPromptRes};
use crate::schema::prompts;
use crate::user::model::User;
use crate::util::web_response::WebErrorResponse;
use crate::{
    db::PgPool,
    openai::{model::OpenAi, res::OpenAiChatRes},
};

use actix_web::{web, HttpResponse};
use diesel::{
    BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Prompt {
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
}

impl Prompt {
    pub async fn new(
        pool: &web::Data<PgPool>,
        body: web::Json<NewPromptReq>,
        is_pay_as_you_go: bool,
    ) -> Result<NewPromptRes, reqwest::Error> {
        let openai_result =
            OpenAi::new_chat_completion(&body.system_prompt, &body.user_prompt).await;

        match openai_result {
            Ok(result) => {
                if is_pay_as_you_go {
                    let user_id = uuid::Uuid::parse_str(&body.user_id).unwrap();
                    let _user_reduce_balance =
                        User::reduce_balance(pool, user_id, result.usage.total_tokens as f64);
                }

                let _prompt_save_res = Self::save_prompt(pool, &body, &result);
                let new_prompt_res =
                    NewPromptRes::new(body.into_inner(), result.choices[0].message.content.clone());
                return Ok(new_prompt_res);
            }
            Err(err) => Err(err),
        }
    }

    pub fn save_prompt(
        pool: &web::Data<PgPool>,
        new_prompt_req: &web::Json<NewPromptReq>,
        openai_chat_res: &OpenAiChatRes,
    ) -> QueryResult<Prompt> {
        let conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&new_prompt_req.user_id).unwrap();
        let prompt_text = format!(
            "{} {}",
            &new_prompt_req.system_prompt, &new_prompt_req.user_prompt
        );

        let data = (
            (prompts::user_id.eq(uuid)),
            (prompts::instruction.eq(&new_prompt_req.system_prompt)),
            (prompts::prompt_token.eq(openai_chat_res.usage.prompt_tokens as i32)),
            (prompts::completion_token.eq(openai_chat_res.usage.completion_tokens as i32)),
            (prompts::prompt_text.eq(prompt_text)),
            (prompts::completion_text.eq(&openai_chat_res.choices[0].message.content)),
            (prompts::total_token.eq(openai_chat_res.usage.total_tokens as i32)),
            (prompts::prompt_type.eq(new_prompt_req.prompt_type.to_string())),
        );

        diesel::insert_into(prompts::table)
            .values(data)
            .get_result(&conn)
    }

    pub async fn new_prompt_response(
        pool: &web::Data<PgPool>,
        body: web::Json<NewPromptReq>,
        is_pay_as_you_go: bool,
    ) -> HttpResponse {
        let result = Prompt::new(&pool, body, is_pay_as_you_go).await;

        match result {
            Ok(new_prompt_res) => HttpResponse::Ok().json(new_prompt_res),
            Err(err) => {
                let err_res = WebErrorResponse::reqwest_server_error(
                    err,
                    "Fail to execute, please try again",
                );
                return HttpResponse::InternalServerError().json(err_res);
            }
        }
    }

    pub fn count_user_monthly_prompt(pool: &web::Data<PgPool>, user_id: &str) -> QueryResult<i64> {
        let conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(user_id).unwrap();

        prompts::table
            .filter(prompts::user_id.eq(uuid).and(prompts::created_at.between(
                diesel::dsl::sql("date_trunc('month', now())"),
                diesel::dsl::sql("now()"),
            )))
            .count()
            .get_result(&conn)
    }

    pub async fn new_monthly_prompt(
        pool: &web::Data<PgPool>,
        body: web::Json<NewPromptReq>,
    ) -> HttpResponse {
        let prompt_count_result = Self::count_user_monthly_prompt(&pool, &body.user_id);

        match prompt_count_result {
            Ok(count) => {
                if count > 5 {
                    let error_res = WebErrorResponse {
                        status: 600,
                        error: "Monthly Limit Exceeded".to_string(),
                        message: "User exceeds monthly limit".to_string(),
                    };
                    return HttpResponse::BadRequest().json(error_res);
                }

                return Self::new_prompt_response(pool, body, false).await;
            }
            Err(_) => {
                let error_res = WebErrorResponse {
                    status: 600,
                    error: "Subscription Not Found".to_string(),
                    message: "User has no subscription".to_string(),
                };
                return HttpResponse::BadRequest().json(error_res);
            }
        }
    }
}
