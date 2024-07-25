use actix_web::{get, post, web, HttpResponse};

use super::model::Student;
use super::request::{FindUserStudentApplicationRequest, NewStudentRequest};
use crate::db::PgPool;
use crate::http_error::HttpError;

#[post("/")]
async fn post_student(
    pool: web::Data<PgPool>,
    request_json: web::Json<NewStudentRequest>,
) -> HttpResponse {
    let request = request_json.into_inner();
    let user_id = uuid::Uuid::parse_str(&request.user_id).unwrap();

    if let Ok(last_application) = Student::find_user_last_application(&pool, &user_id) {
        let can_reapply = request.clone().can_reapply(&last_application);
        if !can_reapply.0 {
            return HttpError::bad_request(&can_reapply.1);
        }
    }

    let application_result = Student::new_application(&pool, &request);
    match application_result {
        Ok(student) => HttpResponse::Created().json(student),
        Err(diesel_error) => HttpError::internal_server_error(&diesel_error.to_string()),
    }
}

#[get("")]
async fn get_user_student_application(
    pool: web::Data<PgPool>,
    request_query: web::Query<FindUserStudentApplicationRequest>,
) -> HttpResponse {
    let request = request_query.into_inner();
    let user_id = uuid::Uuid::parse_str(&request.user_id).unwrap();

    let find_result = Student::find_user_last_active_application(&pool, &user_id);

    match find_result {
        Ok(student) => HttpResponse::Ok().json(student),
        Err(diesel_error) => HttpError::bad_request(&diesel_error.to_string()),
    }
}

pub fn services(config: &mut web::ServiceConfig) {
    config
        .service(post_student)
        .service(get_user_student_application);
}
