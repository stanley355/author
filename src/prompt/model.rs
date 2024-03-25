use super::req::{
    NewImageToTextPromptReq, NewPromptReq, NewTextToSpeechPromptReq, PromptType,
    UpdateImageToTextPromptReq,
};
use super::res::NewPromptRes;
use crate::schema::prompts;
use crate::user::model::User;
use crate::util::web_response::WebErrorResponse;
use crate::{
    db::PgPool,
    openai::{model::OpenAi, req::OpenAiTextToSpeechReq, res::OpenAiChatRes},
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
                        User::reduce_balance(pool, user_id, (result.usage.total_tokens / 2) as f64);
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

    pub fn count_user_monthly_prompt(
        pool: &web::Data<PgPool>,
        user_id: &str,
        prompt_type: &PromptType,
    ) -> QueryResult<i64> {
        let conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(user_id).unwrap();

        prompts::table
            .filter(
                prompts::user_id
                    .eq(uuid)
                    .and(prompts::prompt_type.eq(prompt_type.to_string()))
                    .and(prompts::created_at.between(
                        diesel::dsl::sql("date_trunc('month', now())"),
                        diesel::dsl::sql("now()"),
                    )),
            )
            .count()
            .get_result(&conn)
    }

    pub async fn new_monthly_prompt(
        pool: &web::Data<PgPool>,
        body: web::Json<NewPromptReq>,
    ) -> HttpResponse {
        let prompt_count_result =
            Self::count_user_monthly_prompt(&pool, &body.user_id, &body.prompt_type);

        match prompt_count_result {
            Ok(count) => {
                if count >= 5 {
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

    pub fn save_image_to_text_prompt(
        pool: &web::Data<PgPool>,
        new_prompt_req: &web::Json<NewImageToTextPromptReq>,
    ) -> QueryResult<Prompt> {
        let conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&new_prompt_req.user_id).unwrap();

        let data = (
            (prompts::user_id.eq(uuid)),
            (prompts::instruction.eq("Image to text".to_string())),
            (prompts::prompt_token.eq(0)),
            (prompts::completion_token.eq(0)),
            (prompts::prompt_text.eq("Image to text".to_string())),
            (prompts::completion_text.eq("".to_string())),
            (prompts::total_token.eq(0)),
            (prompts::prompt_type.eq(new_prompt_req.prompt_type.to_string())),
        );

        diesel::insert_into(prompts::table)
            .values(data)
            .get_result(&conn)
    }

    pub async fn new_image_to_text_response(
        pool: &web::Data<PgPool>,
        body: web::Json<NewImageToTextPromptReq>,
    ) -> HttpResponse {
        let result = Self::save_image_to_text_prompt(&pool, &body);

        match result {
            Ok(new_prompt_res) => HttpResponse::Ok().json(new_prompt_res),
            Err(err) => {
                let err_res =
                    WebErrorResponse::server_error(err, "Fail to create prompt, please try again");
                return HttpResponse::InternalServerError().json(err_res);
            }
        }
    }

    pub async fn new_image_to_text_monthly_prompt(
        pool: &web::Data<PgPool>,
        body: web::Json<NewImageToTextPromptReq>,
    ) -> HttpResponse {
        let prompt_count_result =
            Self::count_user_monthly_prompt(&pool, &body.user_id, &body.prompt_type);

        match prompt_count_result {
            Ok(count) => {
                if count >= 5 {
                    let error_res = WebErrorResponse {
                        status: 600,
                        error: "Monthly Limit Exceeded".to_string(),
                        message: "User exceeds monthly limit".to_string(),
                    };
                    return HttpResponse::BadRequest().json(error_res);
                }

                return Self::new_image_to_text_response(pool, body).await;
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

    pub fn update_image_to_text_prompt(
        pool: &web::Data<PgPool>,
        req: &web::Json<UpdateImageToTextPromptReq>,
    ) -> QueryResult<Prompt> {
        let conn = &pool.get().unwrap();

        let completion_token = req.completion_text.split(" ").collect::<Vec<&str>>().len();
        let updated_column = (
            prompts::completion_text.eq(&req.completion_text),
            prompts::completion_token.eq(completion_token as i32),
        );

        diesel::update(prompts::table)
            .filter(prompts::id.eq(&req.prompt_id))
            .set(updated_column)
            .get_result(conn)
    }

    pub async fn update_image_to_text_response(
        pool: &web::Data<PgPool>,
        body: web::Json<UpdateImageToTextPromptReq>,
        is_pay_as_you_go: bool,
    ) -> HttpResponse {
        if is_pay_as_you_go {
            let completion_token = body.completion_text.split(" ").collect::<Vec<&str>>().len();
            let user_id = uuid::Uuid::parse_str(&body.user_id).unwrap();
            let _user_reduce_balance =
                User::reduce_balance(pool, user_id, (completion_token / 2) as f64);
        }
        let result = Self::update_image_to_text_prompt(&pool, &body);

        match result {
            Ok(update_prompt_res) => HttpResponse::Ok().json(update_prompt_res),
            Err(err) => {
                let err_res =
                    WebErrorResponse::server_error(err, "Fail to update prompt, please try again");
                return HttpResponse::InternalServerError().json(err_res);
            }
        }
    }

    pub fn save_text_to_speech_prompt(
        pool: &web::Data<PgPool>,
        new_prompt_req: &web::Json<NewTextToSpeechPromptReq>,
    ) -> QueryResult<Prompt> {
        let conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&new_prompt_req.user_id).unwrap();
        let total_token = &new_prompt_req
            .user_prompt
            .split(" ")
            .collect::<Vec<&str>>()
            .len();

        let data = (
            (prompts::user_id.eq(uuid)),
            (prompts::instruction.eq("Text to Speech".to_string())),
            (prompts::prompt_token.eq(*total_token as i32)),
            (prompts::completion_token.eq(0)),
            (prompts::prompt_text.eq(new_prompt_req.user_prompt.to_string())),
            (prompts::completion_text.eq("".to_string())),
            (prompts::total_token.eq(*total_token as i32)),
            (prompts::prompt_type.eq(PromptType::TextToSpeech.to_string())),
        );

        diesel::insert_into(prompts::table)
            .values(data)
            .get_result(&conn)
    }

    pub async fn new_text_to_speech_response(
        pool: &web::Data<PgPool>,
        body: web::Json<NewTextToSpeechPromptReq>,
        is_pay_as_you_go: bool,
    ) -> HttpResponse {
        if is_pay_as_you_go {
            let completion_token = body.user_prompt.split(" ").collect::<Vec<&str>>().len();
            let user_id = uuid::Uuid::parse_str(&body.user_id).unwrap();
            let _user_reduce_balance =
                User::reduce_balance(pool, user_id, (completion_token / 2) as f64);
        }

        let _save_prompt = Self::save_text_to_speech_prompt(pool, &body);

        let file_req_body = OpenAiTextToSpeechReq::new(&body);
        let file_byte_res = OpenAi::new_text_to_speech(file_req_body).await;

        match file_byte_res {
            Ok(bytes) => HttpResponse::Ok().body(bytes),
            Err(err) => {
                let err_res = WebErrorResponse::reqwest_server_error(
                    err,
                    "Fail to generate file, please try again",
                );
                HttpResponse::InternalServerError().json(err_res)
            }
        }
    }

    pub async fn new_text_to_speech_monthly_prompt(
        pool: &web::Data<PgPool>,
        body: web::Json<NewTextToSpeechPromptReq>,
    ) -> HttpResponse {
        let prompt_count_result =
            Self::count_user_monthly_prompt(&pool, &body.user_id, &PromptType::TextToSpeech);

        match prompt_count_result {
            Ok(count) => {
                if count >= 5 {
                    let error_res = WebErrorResponse {
                        status: 600,
                        error: "Monthly Limit Exceeded".to_string(),
                        message: "User exceeds monthly limit".to_string(),
                    };
                    return HttpResponse::BadRequest().json(error_res);
                }

                return Self::new_text_to_speech_response(pool, body, false).await;
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
