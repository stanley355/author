use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub enum OpenAiAudioSpeechResponseFormat {
  Mp3,
  Opus,
  Wav,
}

impl fmt::Display for OpenAiAudioSpeechResponseFormat{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{:?}", self)
  }
}
