use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct NewCheckbotReq {
    pub user_id: String,
    pub source_text: String,
    pub checkbot_text: String
}
