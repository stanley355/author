use actix_web::web;
use chrono::NaiveDateTime;
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use serde::Serialize;

use crate::{db::PgPool, schema::students};

#[derive(Queryable, Debug, Serialize)]
pub struct Student {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub student_id: String,
    pub student_email: Option<String>,
    pub student_card_img_url: Option<String>,
    pub institution_level: String,
    pub institution_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub free_discount_end_at: NaiveDateTime,
    pub half_discount_end_at: NaiveDateTime,
    pub student_application_valid: bool,
    pub student_application_invalid_reason: Option<String>,
}

impl Student {
    pub fn find_free_discount(pool: &web::Data<PgPool>, user_id: &str) -> QueryResult<Student> {
        let id = uuid::Uuid::parse_str(user_id).unwrap();
        let mut conn = pool.get().unwrap();

        students::table
            .filter(
                students::user_id
                    .eq(id)
                    .and(students::free_discount_end_at.gt(diesel::dsl::sql("now()"))),
            )
            .order_by(students::created_at.desc())
            .get_result::<Student>(&mut conn)
    }
}
