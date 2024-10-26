use crate::db::PgPool;
use crate::http_error::HttpError;
use crate::openai::{OpenAiAudioSpeech, OpenAiRequest, OpenAiRequestEndpoint};
use crate::tts::model::TextToSpeech;
use crate::tts::NewTextToSpeechRequest;
use actix_web::{post, web, HttpResponse};

#[post("/")]
async fn new_tts_service(
    pool: web::Data<PgPool>,
    request_json: web::Json<NewTextToSpeechRequest>,
) -> HttpResponse {
    let request = request_json.into_inner();

    let openai_result = OpenAiAudioSpeech::new_text_to_speech(&request)
        .request_bytes(OpenAiRequestEndpoint::AudioSpeech)
        .await;

    match openai_result {
        Ok(bytes) => {
            let insert_result = TextToSpeech::insert(&pool, &request);
            match insert_result {
                Ok(tts) => {
                    let file_name = format!("{}.{}", &tts.id, &tts.response_format);
                    let file_path = format!("/tmp/{}", file_name);
                    let file_creation = std::fs::write(file_path, &bytes);

                    match file_creation {
                        Ok(_) => HttpResponse::Created().json(tts),
                        Err(create_error) => {
                            HttpError::internal_server_error(&create_error.to_string())
                        }
                    }
                }
                Err(diesel_error) => HttpError::internal_server_error(&diesel_error.to_string()),
            }
        }
        Err(openai_error) => HttpError::internal_server_error(&openai_error.to_string()),
    }
}
pub fn services(config: &mut web::ServiceConfig) {
    config.service(new_tts_service);
}
