
use serde::{Deserialize, Serialize};
use super::enums;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewStudentReq {
    user_id: String,
    student_id: String,
    student_email: Option<String>,
    student_card_img_url: String,
    institution_level: enums::InstitutionLevel,
    institution_name: String,
}
