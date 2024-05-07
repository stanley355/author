#[derive(Debug)]
pub struct OpenAiAudioSpeech {
    model: String,
    input: String,
    voice: String,
}

impl OpenAiAudioSpeech {
    pub fn new(input: String) -> Self {
        Self {
            model: "tts-1".to_string(),
            input,
            voice: "alloy".to_string(),
        }
    }
}
