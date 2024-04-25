use actix_web::web;
use chrono::NaiveDateTime;
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::{db::PgPool, schema::students};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Student {
    id: uuid::Uuid,
    user_id: uuid::Uuid,
    student_id: String,
    student_email: Option<String>,
    student_card_img_url: Option<String>,
    institution_level: String,
    institution_name: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    free_discount_end_at: NaiveDateTime,
    half_discount_end_at: NaiveDateTime,
    student_application_valid: bool,
    student_application_invalid_reason: Option<String>,
}

impl Student {
    pub fn find_active(pool: &web::Data<PgPool>, user_id: &str) -> QueryResult<Student> {
        let id = uuid::Uuid::parse_str(user_id).unwrap();
        let mut conn = pool.get().unwrap();

        students::table
            .filter(
                students::user_id
                    .eq(id)
                    .and(students::student_application_valid.eq(true))
                    .and(students::half_discount_end_at.gt(diesel::dsl::sql("now()"))),
            )
            .order_by(students::created_at.desc())
            .get_result::<Student>(&mut conn)
    }
}
