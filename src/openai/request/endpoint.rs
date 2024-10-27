#[derive(Debug)]
pub enum OpenAiRequestEndpoint {
    ChatCompletion,
    AudioSpeech,
    AudioTranscriptions,
}