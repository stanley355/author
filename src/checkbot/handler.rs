use crate::db::PgPool;
use actix_web::{post, web, HttpResponse};

#[post("/")]
async fn new(pool: web::Data<PgPool>, body: web::Json<>)