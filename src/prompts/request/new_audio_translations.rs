use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NewAudioTranslationsRequest{
    pub user_id: String,
    pub file_url: String,
    pub file_name: String,
    pub temperature: f32,
}