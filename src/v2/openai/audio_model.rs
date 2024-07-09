use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct OpenAiAudioSpeech {
    pub model: String,
    pub input: String,
    pub voice: String,
    // pub speed: Option<f32> // 0.25 - 4.0
}

impl OpenAiAudioSpeech {
    pub fn new(input_text: &str, ) -> Self {
        Self {
            model: "tts-1".to_string(),
            input: input_text.to_string(),
            voice: "alloy".to_string(),
        }
    }
}
