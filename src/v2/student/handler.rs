use actix_web::{get, post, web, HttpResponse};

use super::request::{FindStudentQuery, StudentInstitutionLevel};
use crate::v2::student::model::Student;
use crate::v2::student::request::NewStudentRequestBody;
use crate::{db::PgPool, v2::http_error_response::HttpErrorResponse};

#[post("/")]
async fn new_student(
    pool: web::Data<PgPool>,
    body: web::Json<NewStudentRequestBody>,
) -> HttpResponse {
    let previous_student_subscription = Student::find_active_discount(&pool, &body.user_id);

    if let Ok(student) = previous_student_subscription {
        let last_is_college =
            student.institution_level == StudentInstitutionLevel::College.to_string();
        let last_is_highschool =
            student.institution_level == StudentInstitutionLevel::SeniorHigh.to_string();
        let last_is_juniorschool =
            student.institution_level == StudentInstitutionLevel::JuniorHigh.to_string();
        let apply_highschool =
            body.institution_level.to_string() == StudentInstitutionLevel::SeniorHigh.to_string();
        let apply_juniorschool =
            body.institution_level.to_string() == StudentInstitutionLevel::JuniorHigh.to_string();

        if last_is_college {
            let msg = "Student has previously applied for College Level".to_string();
            return HttpErrorResponse::bad_request(msg);
        }

        if last_is_highschool && (apply_juniorschool || apply_highschool) {
            let msg = "Student has previously applied for Senior High Level".to_string();
            return HttpErrorResponse::bad_request(msg);
        }

        if last_is_juniorschool && apply_juniorschool {
            let msg = "Student has previously applied for Junior High Level".to_string();
            return HttpErrorResponse::bad_request(msg);
        }
    }

    let student_result = Student::new_application(&pool, &body);

    match student_result {
        Ok(student) => HttpResponse::Ok().json(student),
        Err(err) => HttpErrorResponse::internal_server_error(err.to_string()),
    }
}

#[get("")]
async fn find_student(
    pool: web::Data<PgPool>,
    query: web::Query<FindStudentQuery>,
) -> HttpResponse {
    let student_result = Student::find_active_discount(&pool, &query.user_id);

    match student_result {
        Ok(student) => HttpResponse::Ok().json(student),
        Err(err) => HttpErrorResponse::bad_request(err.to_string()),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(new_student).service(find_student);
}
