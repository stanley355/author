use super::model::Prompt;
use super::payment::PromptPayment;
use super::request::{
    DeleteAudioSpeechRequest, NewAudioSpeechPromptRequest, NewPromptRequest, PromptType,
};
use crate::openai::{
    OpenAiAudioSpeech, OpenAiChatCompletionRequest, OpenAiChatCompletionResponse, OpenAiRequest,
    OpenAiRequestEndpoint,
};
use crate::{db::PgPool, http_error::HttpError};
use actix_web::{delete, post, web, HttpResponse};

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

#[post("/audio/speech/")]
async fn post_audio_speech(
    pool: web::Data<PgPool>,
    request_json: web::Json<NewAudioSpeechPromptRequest>,
) -> HttpResponse {
    let request = request_json.into_inner();
    let user_id = uuid::Uuid::parse_str(&request.user_id).unwrap();
    let prompt_payment = Prompt::check_payment(&pool, &user_id, &PromptType::AudioSpeech);

    match prompt_payment {
        PromptPayment::PaymentRequired => HttpError::payment_required(),
        _ => {
            let openai_result = OpenAiAudioSpeech::new(&request)
                .request_bytes(OpenAiRequestEndpoint::AudioSpeech)
                .await;

            match openai_result {
                Ok(bytes) => {
                    let prompt_insert_result = Prompt::new_insert_audio_speech(&pool, &request);
                    match prompt_insert_result {
                        Ok(prompt) => {
                            let file_name = format!("{}.mp3", &prompt.id);
                            let file_path = format!("/tmp/{}", file_name);
                            let file_creation = std::fs::write(file_path, &bytes);

                            match file_creation {
                                Ok(_) => HttpResponse::Created().json(prompt),
                                Err(create_error) => {
                                    HttpError::internal_server_error(&create_error.to_string())
                                }
                            }
                        }
                        Err(diesel_error) => {
                            HttpError::internal_server_error(&diesel_error.to_string())
                        }
                    }
                }
                Err(openai_error) => HttpError::internal_server_error(&openai_error.to_string()),
            }
        }
    }
}

#[delete("/audio/speech")]
async fn delete_audio_speech(query: web::Query<DeleteAudioSpeechRequest>) -> HttpResponse {
    let file_path = format!("/tmp/{}.mp3", &query.prompt_id);
    let file_del_result = std::fs::remove_file(file_path);

    match file_del_result {
        Ok(_) => HttpResponse::Ok().body(query.prompt_id.to_string()),
        Err(err) => HttpError::internal_server_error(&err.to_string()),
    }
}

pub fn services(config: &mut web::ServiceConfig) {
    config
        .service(post_prompt)
        .service(post_audio_speech)
        .service(delete_audio_speech);
}
