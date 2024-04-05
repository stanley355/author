use super::enums;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewStudentReq {
    pub user_id: String,
    pub student_id: String,
    pub student_email: Option<String>,
    pub student_card_img_url: Option<String>,
    pub institution_level: enums::InstitutionLevel,
    pub institution_name: String,
}