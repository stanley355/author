use super::model::Referral;
use super::req::CreateReferralReq;
use crate::{
    db::PgPool,
    user::{model::User, res::ErrorRes},
};

use actix_web::{post, web, HttpResponse};

#[post("/")]
async fn new_referral(pool: web::Data<PgPool>, body: web::Json<CreateReferralReq>) -> HttpResponse {
    let referral = Referral::find(&pool, &body);

    match referral {
        Ok(_) => HttpResponse::BadRequest().json(ErrorRes {
            error: "Referral already exist".to_string(),
            message: "Referral already exist".to_string(),
        }),
        Err(_) => {
            let new_referral = Referral::new(&pool, &body);
            let new_referral_balance = User::increase_referral_balance(&pool, &body);

            match (new_referral, new_referral_balance) {
                (Ok(refer), _) => HttpResponse::Ok().json(refer),
                (Err(refer_error), _) => HttpResponse::InternalServerError().json(ErrorRes {
                    error: refer_error.to_string(),
                    message: "Something went wrong, please try again".to_string(),
                }),
            }
        }
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(new_referral);
}
