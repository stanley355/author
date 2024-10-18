use diesel::{Queryable, QueryResult, RunQueryDsl, ExpressionMethods};
use serde::Serialize;
use actix_web::web;
use crate::db::PgPool;
use crate::openai::{OpenAiAudioTranscriptionsResponse};
use crate::schema::{speech_to_text};
use super::request::NewSpeechToTextRequest;

#[derive(Queryable, Debug, Clone, Serialize)]
pub(super) struct SpeechToText {
    id: i32,
    user_id: uuid::Uuid,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    model: String,
    file_name: String,
    file_url: String,
    language: String,
    transcription: String,
    timestamp_granularity: Option<String>
}

impl SpeechToText {

    pub fn insert(
        pool: &web::Data<PgPool>,
        request: &NewSpeechToTextRequest,
        transcription: &OpenAiAudioTranscriptionsResponse
    ) -> QueryResult<SpeechToText> {
        let user_id = uuid::Uuid::parse_str(&request.user_id).unwrap();

        let timestamp_granularity= match &request.timestamp_granularities {
            Some(granularity) => Some(granularity.to_string()),
            None => None
        };

        let data = (
            (speech_to_text::user_id.eq(user_id)),
            (speech_to_text::model.eq("whisper-1".to_string())),
            (speech_to_text::file_name.eq(&request.file_name)),
            (speech_to_text::file_url.eq(&request.file_url)),
            (speech_to_text::language.eq(&request.language)),
            (speech_to_text::transcription.eq(&transcription.text)),
            (speech_to_text::timestamp_granularity.eq(timestamp_granularity)),
        );

        let mut conn = pool.get().unwrap();
        diesel::insert_into(speech_to_text::table)
            .values(data)
            .get_result(&mut conn)
    }
}