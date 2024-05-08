use actix_web::{post, web, HttpResponse};

use crate::v2::student::model::Student;
use crate::v2::student::request::NewStudentRequestBody;
use crate::{db::PgPool, v2::http_error_response::HttpErrorResponse};

#[post("/")]
async fn new_student(
    pool: web::Data<PgPool>,
    body: web::Json<NewStudentRequestBody>,
) -> HttpResponse {
    let student_result = Student::new_application(&pool, &body);

    match student_result {
        Ok(student) => HttpResponse::Ok().json(student),
        Err(err) => HttpErrorResponse::internal_server_error(err.to_string()),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(new_student);
}
