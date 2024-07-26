use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OpenAiAudioTranslationsResponse {
    pub text: String
}
