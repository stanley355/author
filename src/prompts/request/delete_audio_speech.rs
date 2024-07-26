use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct DeleteAudioSpeechRequest {
    pub(crate) prompt_id: u32,
}
