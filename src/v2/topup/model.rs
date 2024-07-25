use super::request::{TopupPremiumDuration, TopupPremiumRequestBody};
use crate::{
    db::PgPool,
    schema::topups,
    v2::{student::model::Student, subscription::model::Subscription},
};
use actix_web::web;
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, Queryable};
use serde::Serialize;

#[derive(Debug, Serialize, Queryable)]
pub struct TopUp {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub topup_amount: f64,
    pub paid: bool,
    pub topup_type: String,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl TopUp {
    pub fn get_recently_paid(pool: &web::Data<PgPool>, user_id: &str) -> QueryResult<Vec<TopUp>> {
        let mut conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&user_id).unwrap();
        topups::table
            .filter(topups::user_id.eq(uuid).and(topups::paid.eq(true)))
            .order_by(topups::created_at.desc())
            .limit(5)
            .get_results::<TopUp>(&mut conn)
    }

    pub fn calc_premium_price(duration: &TopupPremiumDuration, is_student: bool) -> f64 {
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

    fn new_premium_insert(
        pool: &web::Data<PgPool>,
        user_id: &str,
        amount: &f64,
    ) -> QueryResult<TopUp> {
        let mut conn = pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(user_id).unwrap();

        // default topup_type is topup
        let data = (
            (topups::user_id.eq(uuid)),
            (topups::topup_amount.eq(amount)),
            (topups::topup_type.eq("subscription".to_string())),
        );

        diesel::insert_into(topups::table)
            .values(data)
            .get_result(&mut conn)
    }

    pub fn new_premium(
        pool: &web::Data<PgPool>,
        body: &web::Json<TopupPremiumRequestBody>,
    ) -> Result<TopUp, String> {
        let is_student = {
            let student_result = Student::find_active_discount(pool, &body.user_id);
            match student_result {
                Ok(_) => true,
                Err(_) => false,
            }
        };

        let topup_amount = Self::calc_premium_price(&body.duration, is_student);
        let topup_result = Self::new_premium_insert(pool, &body.user_id, &topup_amount);

        match topup_result {
            Ok(topup) => {
                let subscription_result = Subscription::insert_from_topup(pool, &body, &topup);

                match subscription_result {
                    Ok(_) => Ok(topup),
                    Err(err) => Err(err.to_string()),
                }
            }
            Err(err) => Err(err.to_string()),
        }
    }

    pub fn update_paid_topup(pool: &web::Data<PgPool>, topup_id: &str) -> QueryResult<TopUp> {
        let mut conn = pool.get().unwrap();
        let topup_id = uuid::Uuid::parse_str(topup_id).unwrap();

        diesel::update(topups::table)
            .filter(topups::id.eq(topup_id))
            .set(topups::paid.eq(true))
            .get_result(&mut conn)
    }
}
