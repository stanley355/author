use diesel::{Queryable, QueryResult, RunQueryDsl, ExpressionMethods};
use serde::Serialize;
use actix_web::web;
use crate::db::PgPool;
use crate::schema::{text_to_speech};
use crate::tts::NewTextToSpeechRequest;

#[derive(Queryable, Debug, Clone, Serialize)]
pub(super) struct TextToSpeech {
    pub id: i32,
    user_id: uuid::Uuid,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    model: String,
    input: String,
    voice: String,
    speed: i32,
    pub response_format: String
}

impl TextToSpeech {

    pub fn insert(
        pool: &web::Data<PgPool>,
        request: &NewTextToSpeechRequest,
    ) -> QueryResult<TextToSpeech> {
        let user_id = uuid::Uuid::parse_str(&request.user_id).unwrap();
        let voice = &request.voice.to_string().to_lowercase();
        let response_format = &request.response_format.to_string().to_lowercase();

        let data = (
            (text_to_speech::user_id.eq(user_id)),
            (text_to_speech::model.eq("tts-1".to_string())),
            (text_to_speech::input.eq(&request.input)),
            (text_to_speech::voice.eq(voice)),
            (text_to_speech::speed.eq(request.speed as i32)),
            (text_to_speech::response_format.eq(response_format))
        );

        let mut conn = pool.get().unwrap();
        diesel::insert_into(text_to_speech::table)
            .values(data)
            .get_result(&mut conn)
    }
}
