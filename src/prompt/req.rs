use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PromptType {
    Translate,
    GrammarCheck,
    ImageToText,
    TextToSpeech
}

impl fmt::Display for PromptType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewPromptReq {
    pub user_id: String,
    pub system_prompt: String,
    pub user_prompt: String,
    pub prompt_type: PromptType,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewImageToTextPromptReq {
    pub user_id: String,
    pub prompt_type: PromptType
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateImageToTextPromptReq {
    pub prompt_id: i32,
    pub user_id: String,
    pub completion_text: String 
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TextToSpeechVoice {
    Alloy,
    Echo,
    Fable,
    Onyx,
    Nova,
    Shimmer
}

impl fmt::Display for TextToSpeechVoice{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewTextToSpeechPromptReq {
    pub user_id: String,
    pub user_prompt: String,
    pub voice: TextToSpeechVoice
}


pub enum MontlyPromptReq {
    NewPromptReq(NewPromptReq),
    NewImageToTextPromptReq(NewImageToTextPromptReq),
    NewTextToSpeechPromptReq(NewTextToSpeechPromptReq)
}
