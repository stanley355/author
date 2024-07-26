use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum OpenAiAudioSpeechVoice {
    Alloy,
    Echo,
    Fable,
    Onyx,
    Nova,
    Shimmer,
}