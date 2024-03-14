use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};

use super::req::NewPromptReq;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewPromptRes {
    pub user_id: String,
    pub status: u16,
    pub status_message: String,
    pub completion_text: String,
}

impl NewPromptRes {
    pub fn new(req: NewPromptReq, completion_text: String) -> Self {
        NewPromptRes {
            user_id: req.user_id,
            status: StatusCode::OK.as_u16(),
            status_message: "OK".to_string(),
            completion_text,
        }
    }
}
