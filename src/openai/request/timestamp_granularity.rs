use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize, Clone)]
pub enum OpenAiAudioTranscriptionsTimestampGranularity {
    Word,
    Segment,
}

impl fmt::Display for OpenAiAudioTranscriptionsTimestampGranularity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
