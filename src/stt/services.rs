use crate::db::PgPool;
use crate::openai::{OpenAiAudioTranscriptionsRequest, OpenAiAudioTranscriptionsResponse, OpenAiRequest, OpenAiRequestEndpoint};
use actix_web::{post, web, HttpResponse};
use crate::http_error::HttpError;
use crate::schema::speech_to_text::transcription;
use crate::stt::model::SpeechToText;
use crate::stt::NewSpeechToTextRequest;
// use crate::translation::model::Translation;
// use crate::translation::request::NewTranslationRequest;

#[post("/")]
pub async fn new_speech_to_text_service(
    pool: web::Data<PgPool>,
    request_json: web::Json<NewSpeechToTextRequest>,
) -> HttpResponse {
    let request = request_json.into_inner();
    let openai_request_form_data =
        OpenAiAudioTranscriptionsRequest::new(&request).await;

    match openai_request_form_data {
        Ok(form_data) => {
            let openai_result = OpenAiAudioTranscriptionsRequest::request_multipart::<
                OpenAiAudioTranscriptionsResponse,
            >(
                form_data, OpenAiRequestEndpoint::AudioTranscriptions
            )
                .await;

            match openai_result {
                Ok(transcriptions) => {
                    let insert_result = SpeechToText::insert(&pool, &request, &transcriptions);

                    match insert_result {
                        Ok(_) => HttpResponse::Ok().json(transcriptions),
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
        Err(file_error) => HttpError::bad_request(&file_error.to_string()),
    }
}

pub fn services(config: &mut web::ServiceConfig) {
    config.service(new_speech_to_text_service);
}
