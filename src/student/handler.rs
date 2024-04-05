use actix_web::{post, web, HttpResponse};

use super::model::Student;
use super::req::*;
use crate::db::PgPool;
use crate::util::http_error_response::HttpErrorResponse;

#[post("/")]
async fn new_student(
    pool: web::Data<PgPool>,
    new_student_req: web::Json<NewStudentReq>,
) -> HttpResponse {
    if &new_student_req.student_id == "" || &new_student_req.institution_name == "" {
        let msg = "Missing student id or institution name";
        return HttpErrorResponse::new(Some(400), msg.to_string(), msg).response();
    }

    let new_student_result = Student::insert_one(pool, new_student_req.into_inner());

    match new_student_result {
        Ok(student) => HttpResponse::Ok().json(student),
        Err(err) => {
            let msg = "Fail to add student, please try again";
            HttpErrorResponse::new(None, err.to_string(), msg).response()
        }
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(new_student);
}
