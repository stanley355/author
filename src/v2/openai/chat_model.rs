use std::fmt;

#[derive(Debug, Clone)]
pub enum OpenAiChatRole {
  System,
  User,
}

impl fmt::Display for OpenAiChatRole {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{:?}", self)
  }
}

#[derive(Debug, Clone)]
pub struct OpenAiChatMessage {
  role: OpenAiChatRole,
  content: String
}

#[derive(Debug, Clone)]
pub struct OpenAiChat {
  model: String,
  messages: Vec<OpenAiChatMessage>
}