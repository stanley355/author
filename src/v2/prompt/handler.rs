use crate::v2::prompt::request::NewPromptRequestBody;
use crate::{db::PgPool, v2::user::model::User};
use actix_web::{post, web, HttpResponse};

#[post("/")]
async fn new_prompt(
    pool: web::Data<PgPool>,
    body: web::Json<NewPromptRequestBody>,
) -> HttpResponse {
    let prompt_payment = User::check_prompt_payment(&pool, &body.user_id, &body.prompt_type);
    println!("{:?}", prompt_payment);
    HttpResponse::Ok().body("".to_string())
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(new_prompt);
}
