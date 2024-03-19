use super::model::Prompt;
use super::req::NewPromptReq;
use crate::{
    db::PgPool, subscription::model::Subscription, user::model::User,
    util::web_response::WebErrorResponse,
};
use actix_web::{post, web, HttpResponse};

#[post("/")]
async fn new_prompt(pool: web::Data<PgPool>, body: web::Json<NewPromptReq>) -> HttpResponse {
    let subscription_result = Subscription::find_active_subscription(&pool, &body.user_id);

    match subscription_result {
        Ok(_) => Prompt::new_prompt_response(&pool, body, false).await,
        Err(_) => {
            let user_result = User::find_by_id(&pool, &body.user_id);

            match user_result {
                Ok(user) => match user.balance > 0.0 {
                    true => Prompt::new_prompt_response(&pool, body, true).await,
                    false => Prompt::new_monthly_prompt(&pool, body).await
                },
                Err(err) => {
                    let err_res = WebErrorResponse::server_error(err, "User not found");
                    return HttpResponse::BadRequest().json(err_res);
                }
            }
        }
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(new_prompt);
}
