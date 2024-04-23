use actix_web::web;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use super::req::{TopupPaidReq, TopupPayasyougoReq, TopupPremiumDuration, TopupPremiumReq};
use crate::db::PgPool;
use crate::schema::topups;
use crate::student::model::Student;

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct TopUp {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub topup_amount: f64,
    pub paid: bool,
    pub topup_type: String,
}

impl TopUp {
    pub fn new_payasyougo(
        pool: &web::Data<PgPool>,
        body: &web::Json<TopupPayasyougoReq>,
    ) -> QueryResult<TopUp> {
        let mut conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&body.user_id).unwrap();
        let data = (
            (topups::user_id.eq(uuid)),
            (topups::topup_amount.eq(&body.topup_amount)),
        );

        diesel::insert_into(topups::table)
            .values(data)
            .get_result(&mut conn)
    }

    pub fn calc_timely_price(duration: &TopupPremiumDuration, is_student: bool) -> f64 {
        if is_student {
            return match duration {
                TopupPremiumDuration::Monthly => 12500.0,
                TopupPremiumDuration::Quarterly => 30000.0,
                TopupPremiumDuration::HalfYearly => 70000.0,
            };
        }

        match duration {
            TopupPremiumDuration::Monthly => 25000.0,
            TopupPremiumDuration::Quarterly => 70000.0,
            TopupPremiumDuration::HalfYearly => 150000.0,
        }
    }

    pub fn check_premium_price(
        pool: &web::Data<PgPool>,
        body: &web::Json<TopupPremiumReq>,
    ) -> f64 {
        let student_disc_check = Student::find_active_application(pool, &body.user_id);

        match student_disc_check {
            Ok(student) => {
                let student_disc_availability = student.check_discount_availability();
                match (
                    student_disc_availability.is_student,
                    student_disc_availability.is_half_discount,
                ) {
                    (true, true) => Self::calc_timely_price(&body.duration, true),
                    (_, _) => Self::calc_timely_price(&body.duration, false),
                }
            }
            Err(_) => Self::calc_timely_price(&body.duration, false),
        }
    }

    pub fn new_premium(
        pool: &web::Data<PgPool>,
        body: &web::Json<TopupPremiumReq>,
    ) -> QueryResult<TopUp> {
        let price = Self::check_premium_price(pool, body);

        let mut conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&body.user_id).unwrap();

        let data = (
            (topups::user_id.eq(uuid)),
            (topups::topup_amount.eq(&price)),
            (topups::topup_type.eq("subscription".to_string())),
        );

        diesel::insert_into(topups::table)
            .values(data)
            .get_result(&mut conn)
    }

    pub fn find_user_topups(pool: &web::Data<PgPool>, user_id: &str) -> QueryResult<Vec<TopUp>> {
        let mut conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&user_id).unwrap();
        topups::table
            .filter(topups::user_id.eq(uuid))
            .order_by(topups::created_at.desc())
            .limit(5)
            .get_results::<TopUp>(&mut conn)
    }

    pub fn update_paid_topup(pool: &web::Data<PgPool>, body: &TopupPaidReq) -> QueryResult<TopUp> {
        let mut conn = pool.get().unwrap();
        let topup_id = uuid::Uuid::parse_str(&body.id).unwrap();

        diesel::update(topups::table)
            .filter(topups::id.eq(topup_id))
            .set(topups::paid.eq(true))
            .get_result(&mut conn)
    }
}
