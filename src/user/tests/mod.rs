use crate::db;
use std::env;
use diesel::RunQueryDsl;
use diesel::{Connection, PgConnection};

// Keep the databse info in mind to drop them later
struct TestContext {
    base_url: String,
    db_name: String,
}

impl TestContext {
    fn new(base_url: &str, db_name: &str) -> Self {
        // First, connect to postgres db to be able to create our test
        // database.
        let postgres_url = format!("{}/postgres", base_url);
        let conn =
            PgConnection::establish(&postgres_url).expect("Cannot connect to postgres database.");

        // Create a new database for the test
        let query = diesel::sql_query(format!("SELECT * FROM users").as_str());
        query
            .execute(&conn)
            .expect(format!("Could not create database {}", db_name).as_str());

        Self {
            base_url: base_url.to_string(),
            db_name: db_name.to_string(),
        }
    }
}


#[test]
fn build_pool() {
    // Needs to be created first.
    // let _ctx = TestContext::new("postgres://postgres:password@localhost/author", "users");

    let db_url = "postgres://postgres:password@localhost/author".to_string();
    let pool = db::build_pool(&db_url);
    // Do your test here
}
