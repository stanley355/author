use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub(crate) enum PromptType {
    Translate,
    Checkbot,
    AudioSpeech, // old: TextToSpeech,
    AudioTranscriptions, //old: Transcriptions,
    AudioTranslations,
    PhoneticTranscriptions,
}

impl fmt::Display for PromptType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
