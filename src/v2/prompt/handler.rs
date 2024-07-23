use super::request::DeleteTtsFileQuery;
use super::response::PromptHttpResponse;
use crate::v2::http_error_response::HttpErrorResponse;
use crate::v2::prompt::prompt_payment::PromptPayment;
use crate::v2::prompt::request::NewPromptRequestBody;
use crate::v2::prompt::request::NewTextToSpeechRequestBody;
use crate::v2::prompt::request::NewTranscriptionsRequestBody;
use crate::v2::prompt::request::PromptType;
use crate::{db::PgPool, v2::user::model::User};
use actix_web::{delete, post, web, HttpResponse};

#[post("/")]
async fn new_prompt(
    pool: web::Data<PgPool>,
    body: web::Json<NewPromptRequestBody>,
) -> HttpResponse {
    let prompt_payment = User::check_prompt_payment(&pool, &body.user_id, &body.prompt_type);

    return match prompt_payment {
        PromptPayment::PaymentRequired => HttpErrorResponse::payment_required(),
        PromptPayment::Balance => PromptHttpResponse::new_instruct(&pool, &body, true).await,
        _ => PromptHttpResponse::new_instruct(&pool, &body, false).await,
    };
}

#[post("/tts/")]
async fn new_text_to_speech(
    pool: web::Data<PgPool>,
    body: web::Json<NewTextToSpeechRequestBody>,
) -> HttpResponse {
    let prompt_payment =
        User::check_prompt_payment(&pool, &body.user_id, &PromptType::TextToSpeech);

    return match prompt_payment {
        PromptPayment::PaymentRequired => HttpErrorResponse::payment_required(),
        PromptPayment::Balance => PromptHttpResponse::new_text_to_speech(&pool, &body, true).await,
        _ => PromptHttpResponse::new_text_to_speech(&pool, &body, false).await,
    };
}

#[delete("/tts/file")]
async fn delete_tts_file(query: web::Query<DeleteTtsFileQuery>) -> HttpResponse {
    let file_path = format!("/tmp/{}.mp3", &query.prompt_id);
    let file_del_result = std::fs::remove_file(file_path);

    match file_del_result {
        Ok(_) => HttpResponse::Ok().body(query.prompt_id.to_string()),
        Err(err) => HttpErrorResponse::internal_server_error(err.to_string()),
    }
}

#[post("/transcriptions/")]
async fn new_transcriptions(
    pool: web::Data<PgPool>,
    body: web::Json<NewTranscriptionsRequestBody>,
) -> HttpResponse {
    let prompt_payment =
        User::check_prompt_payment(&pool, &body.user_id, &PromptType::Transcriptions);

    return match prompt_payment {
        PromptPayment::PaymentRequired => HttpErrorResponse::payment_required(),
        _ => PromptHttpResponse::new_transcriptions(&pool, &body).await,
    };
}

#[post("/stream/")]
async fn new_prompt_stream(
    pool: web::Data<PgPool>,
    body: web::Json<NewPromptRequestBody>,
) -> HttpResponse {
    PromptHttpResponse::new_instruct_stream(&pool, &body).await
}

pub fn route(config: &mut web::ServiceConfig) {
    config
        .service(new_prompt)
        .service(new_text_to_speech)
        .service(delete_tts_file)
        .service(new_transcriptions)
        .service(new_prompt_stream);
}
