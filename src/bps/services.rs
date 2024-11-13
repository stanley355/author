use actix_web::{post, web, HttpResponse};
use actix_web::web::Json;
use crate::bps::request::BpsFaqRequest;
use crate::http_error::HttpError;
use crate::openai::{OpenAiChatCompletionRequest, OpenAiChatCompletionResponse, OpenAiRequest, OpenAiRequestEndpoint};

#[post("/faq/")]
pub async fn bps_faq_service(
    request_json: Json<BpsFaqRequest>,
) -> HttpResponse {
    let request = request_json.into_inner();

    let openai_request = OpenAiChatCompletionRequest::new_faq(request).request_json::<OpenAiChatCompletionResponse>(OpenAiRequestEndpoint::ChatCompletion).await;

    match openai_request {
       Ok(openai_response) =>{
          let response_message = &openai_response.choices[0].message;
           HttpResponse::Ok().json(response_message)
       },
        Err(e) => HttpError::internal_server_error(&e.to_string()),
    }
}

pub fn services(config: &mut web::ServiceConfig) {
    config.service(bps_faq_service);
}