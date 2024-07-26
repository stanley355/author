use super::model::Prompt;
use super::payment::PromptPayment;
use super::request::{NewPromptRequest, PromptType};
use crate::openai::{
    OpenAiChatCompletionRequest, OpenAiChatCompletionResponse, OpenAiRequest, OpenAiRequestEndpoint,
};
use crate::{db::PgPool, http_error::HttpError};
use actix_web::{post, web, HttpResponse};

#[post("/")]
async fn post_prompt(
    pool: web::Data<PgPool>,
    request_json: web::Json<NewPromptRequest>,
) -> HttpResponse {
    let request = request_json.into_inner();

    match &request.prompt_type {
        PromptType::Translate | PromptType::Checkbot | PromptType::PhoneticTranscriptions => {
            let user_id = uuid::Uuid::parse_str(&request.user_id).unwrap();
            let prompt_payment = Prompt::check_payment(&pool, &user_id, &request.prompt_type);

            match prompt_payment {
                PromptPayment::PaymentRequired => HttpError::payment_required(),
                _ => {
                    let openai_result = OpenAiChatCompletionRequest::new(&request)
                        .request_json::<OpenAiChatCompletionResponse>(
                            OpenAiRequestEndpoint::ChatCompletion,
                        )
                        .await;

                    match openai_result {
                        Ok(chat_completion_response) => {
                            let prompts_insert_result = Prompt::new_insert_chat_completion(
                                &pool,
                                &request,
                                &chat_completion_response,
                            );
                            match prompts_insert_result {
                                Ok(prompts) => HttpResponse::Ok().json(prompts),
                                Err(diesel_error) => {
                                    HttpError::internal_server_error(&diesel_error.to_string())
                                }
                            }
                        }
                        Err(openai_error) => {
                            HttpError::internal_server_error(&openai_error.to_string())
                        }
                    }
                }
            }
        }
        _ => {
            let msg = "Only Translate, Checkbot, and PhoneticTranscriptions prompt_type accepted";
            HttpError::bad_request(msg)
        }
    }
}

pub fn services(config: &mut web::ServiceConfig) {
    config.service(post_prompt);
}
