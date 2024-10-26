use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NewCheckbotRequest {
    pub user_id: String,
    pub instruction: String,
    pub system_content: String,
    pub user_content: String,
    pub n: Option<u32>,
    pub temperature: Option<f32>,
}
