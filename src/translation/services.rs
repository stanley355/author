use crate::db::PgPool;
use crate::openai::{
    OpenAiChatCompletionRequest, OpenAiChatCompletionResponse, OpenAiRequest, OpenAiRequestEndpoint,
};
use actix_web::{post, web, HttpResponse};
use crate::http_error::HttpError;
use crate::translation::model::Translation;
use crate::translation::request::NewTranslationRequest;

#[post("/")]
pub async fn new_translation_service(
    pool: web::Data<PgPool>,
    request_json: web::Json<NewTranslationRequest>,
) -> HttpResponse {
    let request = request_json.into_inner();
    let openai_result = OpenAiChatCompletionRequest::new_translation(&request)
        .request_json::<OpenAiChatCompletionResponse>(OpenAiRequestEndpoint::ChatCompletion)
        .await;
    match openai_result {
        Ok(openai) => {
            let insert_result = Translation::insert(&pool, &request, &openai);

            match insert_result {
                Ok(inserted) => HttpResponse::Ok().json(inserted),
                Err(insert_error) => HttpError::internal_server_error(&insert_error.to_string())
            }
        },
        Err(openai_err) => HttpError::internal_server_error(&openai_err.to_string())
    }
}

pub fn services(config: &mut web::ServiceConfig) {
    config.service(new_translation_service);
}
