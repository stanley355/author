use super::model::User;
use actix_web::web::Data;

#[test]
fn user_checks() {
    let db_url = "postgres://postgres:password@localhost/author".to_string();
    let pool = crate::db::build_pool(&db_url);
    let user_check = User::check_user(Data::new(pool.unwrap()), "stanley.winata@lifepal.co.id".to_string());
    assert_eq!(user_check.unwrap().email, "stanley.winata@lifepal.co.id".to_string());
}
