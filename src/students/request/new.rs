use serde::Deserialize;
use std::fmt;

use crate::students::model::Student;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub(crate) enum StudentInstitutionLevel {
    JuniorHigh,
    SeniorHigh,
    College,
}

impl fmt::Display for StudentInstitutionLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct NewStudentRequest {
    pub user_id: String,
    pub student_id: String,
    pub student_email: Option<String>,
    pub student_card_img_url: Option<String>,
    pub institution_level: StudentInstitutionLevel,
    pub institution_name: String,
}

impl NewStudentRequest {
    // (bool, String) -> (is_valid, invalid_reason)
    pub(crate) fn can_reapply(self, last_application: &Student) -> (bool, String) {
        let new_institution_level = self.institution_level.to_string();
        let last_institution_level = last_application.institution_level.clone();

        if last_institution_level == "College".to_string() {
            return (
                false,
                "Application not permitted. Student previously applied for College Level"
                    .to_string(),
            );
        }

        if last_institution_level == "SeniorHigh".to_string() {
            return (
                new_institution_level == "JuniorHigh".to_string()
                    || new_institution_level == "SeniorHigh".to_string(),
                "Application not permitted. Student previously applied for Senior High Level"
                    .to_string(),
            );
        }

        if last_institution_level == "JuniorHigh".to_string() {
            return (
                new_institution_level == "JuniorHigh".to_string(),
                "Application not permitted. Student previously applied for Junior High Level"
                    .to_string(),
            );
        }

        return (true, "".to_string());
    }
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct FindUserStudentApplicationRequest {
    pub user_id: String,
}
