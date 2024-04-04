use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Student {
    id: Uuid,
    user_id: Uuid,
    student_id: String,
    student_email: Option<String>,
    student_card_img_url: String,
    institution_level: String,
    institution_name: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    free_discount_end_at: NaiveDateTime,
    half_discount_end_at: NaiveDateTime,
    student_application_valid: bool,
    student_application_invalid_reason: Option<String>,
}
