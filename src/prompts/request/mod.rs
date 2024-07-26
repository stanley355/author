mod delete_audio_speech;
mod new;
mod new_audio_speech;
mod new_audio_transcriptions;
mod prompt_type;

pub(crate) use delete_audio_speech::DeleteAudioSpeechRequest;
pub use new::NewPromptRequest;
pub use new_audio_speech::NewAudioSpeechPromptRequest;
pub use new_audio_transcriptions::NewAudioTranscriptionsRequest;
pub(crate) use prompt_type::*;
