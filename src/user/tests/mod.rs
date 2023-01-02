#[test]
fn user_checks() {
    let db_url = "postgres://postgres:password@localhost/author".to_string();
    let pool = crate::db::build_pool(&db_url);
    let default_email = "stanley.winata@lifepal.co.id".to_string();
    let user_check = super::model::User::check_user(actix_web::web::Data::new(pool.unwrap()), default_email.clone());

    match user_check {
        Ok(user) => assert_eq!(user.email, default_email),
        Err(_) => assert_eq!("DB connection failure", default_email)
    }
}
