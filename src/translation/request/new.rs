use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NewTranslationRequest {
    pub user_id: String,
    pub content_language: String,
    pub target_language: String,
    pub system_content: String,
    pub user_content: String,
    pub n: Option<u32>,
    pub temperature: Option<f32>,
}
