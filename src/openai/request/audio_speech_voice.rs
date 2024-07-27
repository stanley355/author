use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub enum OpenAiAudioSpeechVoice {
    Alloy,
    Echo,
    Fable,
    Onyx,
    Nova,
    Shimmer,
}

impl fmt::Display for OpenAiAudioSpeechVoice {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{:?}", self)
  }
}
