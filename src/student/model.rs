use super::req::NewStudentReq;
use super::res::StudentDiscountAvailabilityRes;
use crate::diesel::ExpressionMethods;
use crate::{db::PgPool, schema::students};

use actix_web::web;
use chrono::{Duration, NaiveDateTime, Utc};
use diesel::{BoolExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Student {
    id: Uuid,
    user_id: Uuid,
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
    fn calc_discount_end_at(is_free_discount: bool) -> NaiveDateTime {
        let days = match is_free_discount {
            true => 366,
            false => 732,
        };

        let current_time = Utc::now();
        let end_time = current_time
            .checked_add_signed(Duration::days(days))
            .unwrap();
        return end_time.naive_utc();
    }

    pub fn insert_one(
        pool: web::Data<PgPool>,
        new_student_req: NewStudentReq,
    ) -> QueryResult<Student> {
        let user_id = Uuid::parse_str(&new_student_req.user_id).unwrap();
        let insti_level = &new_student_req.institution_level.to_string();
        let free_disc_end_at = Self::calc_discount_end_at(true);
        let half_disc_end_at = Self::calc_discount_end_at(false);

        let data = (
            (students::user_id.eq(user_id)),
            (students::student_id.eq(&new_student_req.student_id)),
            (students::student_email.eq(&new_student_req.student_email)),
            (students::student_card_img_url.eq(&new_student_req.student_card_img_url)),
            (students::institution_level.eq(insti_level)),
            (students::institution_name.eq(&new_student_req.institution_name)),
            (students::free_discount_end_at.eq(free_disc_end_at)),
            (students::half_discount_end_at.eq(half_disc_end_at)),
        );

        let conn = pool.get().unwrap();
        diesel::insert_into(students::table)
            .values(data)
            .get_result::<Student>(&conn)
    }

    pub fn find_active_application(
        pool: &web::Data<PgPool>,
        user_id: &str,
    ) -> QueryResult<Student> {
        let user_id = Uuid::parse_str(user_id).unwrap();
        let conn = pool.get().unwrap();

        students::table
            .filter(
                students::user_id
                    .eq(user_id)
                    .and(students::student_application_valid.eq(true))
                    .and(students::half_discount_end_at.gt(diesel::dsl::sql("now()"))),
            )
            .order_by(students::created_at.desc())
            .get_result::<Student>(&conn)
    }

    pub fn find_last_application(pool: &web::Data<PgPool>, user_id: &str) -> QueryResult<Student> {
        let user_id = Uuid::parse_str(user_id).unwrap();
        let conn = pool.get().unwrap();

        students::table
            .filter(
                students::user_id
                    .eq(user_id)
                    .and(students::student_application_valid.eq(true)),
            )
            .order_by(students::created_at.desc())
            .get_result::<Student>(&conn)
    }

    pub fn check_discount_availability(self) -> StudentDiscountAvailabilityRes {
        let free_disc_timestamp = self.free_discount_end_at.timestamp();
        let half_disc_timestamp = self.half_discount_end_at.timestamp();
        let current_timestamp = Utc::now().timestamp();
        StudentDiscountAvailabilityRes {
            is_student: self.student_application_valid,
            is_free_discount: free_disc_timestamp > current_timestamp,
            is_half_discount: half_disc_timestamp > current_timestamp,
        }
    }

    pub fn find_student_college_subscription(
        pool: &web::Data<PgPool>,
        user_id: &str,
    ) -> QueryResult<Student> {
        let user_id = Uuid::parse_str(user_id).unwrap();
        let conn = pool.get().unwrap();

        students::table
            .filter(
                students::user_id
                    .eq(user_id)
                    .and(students::institution_level.eq("College")),
            )
            .order_by(students::created_at.desc())
            .get_result::<Student>(&conn)
    }
}
