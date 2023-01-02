#[allow(dead_code)]
fn create_pool_data() -> actix_web::web::Data<crate::db::PgPool> {
    let db_url = "postgres://postgres:password@localhost/author".to_string();
    let pool = crate::db::build_pool(&db_url).unwrap();
    actix_web::web::Data::new(pool)
}

#[test]
fn user_checks() {
    let pool = create_pool_data();
    let default_email = "stanley.winata@lifepal.co.id".to_string();
    let user_check = super::model::User::check_user(pool, default_email.clone());

    match user_check {
        Ok(user) => assert_eq!(user.email, default_email),
        Err(_) => assert_eq!("DB connection failure", default_email),
    }
}

#[test]
fn update_has_channel_check() {
    let pool = create_pool_data();
    let test_email = "unit@test.com".to_string();
    let update_req = super::req::UpdateUserReq {
        fullname: "unit_test".to_string(),
        email: test_email.clone(),
        password: Some("".to_string()),
        phone_number: Some("".to_string()),
        has_channel: Some(true)
    };

    let user_check = super::model::User::update_has_channel(pool, actix_web::web::Json(update_req));

    match user_check {
        Ok(user) => assert_eq!(user.has_channel, true),
        Err(_) => assert_eq!("DB connection failure", test_email),
    }
}
