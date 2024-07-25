use actix_web::{post, web, HttpResponse};

use super::model::Student;
use super::request::NewStudentRequest;
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

pub fn services(config: &mut web::ServiceConfig) {
    config.service(post_student);
}
