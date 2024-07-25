use actix_web::web;
use diesel::{QueryResult, Queryable, ExpressionMethods, RunQueryDsl, QueryDsl};
use serde::Serialize;

use crate::db::PgPool;
use crate::schema::students;

#[derive(Queryable, Debug, Serialize)]
pub(super) struct Student {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub student_id: String,
    pub student_email: Option<String>,
    pub student_card_img_url: Option<String>,
    pub institution_level: String,
    pub institution_name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub free_discount_end_at: chrono::NaiveDateTime,
    pub half_discount_end_at: chrono::NaiveDateTime,
}

impl Student {
    pub fn find_user_last_application(
        pool: &web::Data<PgPool>,
        user_id: &uuid::Uuid,
    ) -> QueryResult<Student> {
        let mut conn = pool.get().unwrap();

        students::table
            .filter(students::user_id.eq(user_id))
            .order_by(students::created_at.desc())
            .get_result::<Student>(&mut conn)
    }
}
