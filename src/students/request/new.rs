use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub enum StudentInstitutionLevel {
    JuniorHigh,
    SeniorHigh,
    College,
}

impl fmt::Display for StudentInstitutionLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Deserialize)]
pub struct NewStudentRequest {
    pub user_id: String,
    pub student_id: String,
    pub student_email: Option<String>,
    pub student_card_img_url: Option<String>,
    pub institution_level: StudentInstitutionLevel,
    pub institution_name: String,
}
