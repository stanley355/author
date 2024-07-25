use actix_web::{get, post, web, HttpResponse};

use crate::db::PgPool;

#[post("/")]
async fn post_student(
    pool: web::Data<PgPool>,
    // body: web::Json<NewStudentRequestBody>,
) -> HttpResponse {
    HttpResponse::Created().body("woi")
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(post_student);
}
