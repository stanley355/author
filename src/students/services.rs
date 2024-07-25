use actix_web::{get, post, web, HttpResponse};

use super::model::Student;
use super::request::NewStudentRequest;
use crate::db::PgPool;

#[post("/")]
async fn post_student(
    pool: web::Data<PgPool>,
    request_json: web::Json<NewStudentRequest>,
) -> HttpResponse {
    let request = request_json.into_inner();
    let user_id = uuid::Uuid::parse_str(&request.user_id).unwrap();
    let last_application_result = Student::find_user_last_application(&pool, &user_id);

    match last_application_result {
        Ok(_) => HttpResponse::Created().body("woi"),
        Err(_) => HttpResponse::Ok().body("woi"),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(post_student);
}
