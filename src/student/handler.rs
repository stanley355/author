use actix_web::{get, post, web, HttpResponse};

use super::model::Student;
use super::req::*;
use crate::db::PgPool;
use crate::user::req::UserIdReq;
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

#[get("/availability")]
async fn check_discount_availability(
    pool: web::Data<PgPool>,
    query: web::Query<UserIdReq>,
) -> HttpResponse {
    let student_availability_result = Student::find_active_discount(&pool, &query.user_id);

    match student_availability_result {
        Ok(student) => HttpResponse::Ok().json(student.check_discount_availability()),
        Err(err) => {
            let msg = "Fail to check, please try again";
            HttpErrorResponse::new(None, err.to_string(), msg).response()
        }
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config
        .service(new_student)
        .service(check_discount_availability);
}
