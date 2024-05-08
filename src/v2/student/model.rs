use actix_web::web;
use chrono::{Duration, NaiveDateTime, Utc};
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use serde::Serialize;

use crate::{db::PgPool, schema::students};

use super::request::NewStudentRequestBody;

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

    pub fn find_active_discount(pool: &web::Data<PgPool>, user_id: &str) -> QueryResult<Student> {
        let id = uuid::Uuid::parse_str(user_id).unwrap();
        let mut conn = pool.get().unwrap();

        students::table
            .filter(
                students::user_id
                    .eq(id)
                    .and(students::half_discount_end_at.gt(diesel::dsl::sql("now()"))),
            )
            .order_by(students::created_at.desc())
            .get_result::<Student>(&mut conn)
    }

    fn calc_free_disc_end_at() -> NaiveDateTime {
        let end_time = Utc::now().checked_add_signed(Duration::days(365)).unwrap();
        return end_time.naive_utc();
    }

    fn calc_half_disc_end_at() -> NaiveDateTime {
        let end_time = Utc::now().checked_add_signed(Duration::days(730)).unwrap();
        return end_time.naive_utc();
    }

    pub fn new_application(
        pool: &web::Data<PgPool>,
        body: &web::Json<NewStudentRequestBody>,
    ) -> QueryResult<Student> {
        let user_id = uuid::Uuid::parse_str(&body.user_id).unwrap();
        let insti_level = &body.institution_level.to_string();
        let free_disc_end_at = Self::calc_free_disc_end_at();
        let half_disc_end_at = Self::calc_half_disc_end_at();

        let data = (
            (students::user_id.eq(user_id)),
            (students::student_id.eq(&body.student_id)),
            (students::student_email.eq(&body.student_email)),
            (students::student_card_img_url.eq(&body.student_card_img_url)),
            (students::institution_level.eq(insti_level)),
            (students::institution_name.eq(&body.institution_name)),
            (students::free_discount_end_at.eq(free_disc_end_at)),
            (students::half_discount_end_at.eq(half_disc_end_at)),
        );

        let mut conn = pool.get().unwrap();
        diesel::insert_into(students::table)
            .values(data)
            .get_result::<Student>(&mut conn)
    }
}
