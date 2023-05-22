#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv::dotenv;
use std::env;

mod db;
mod schema;
mod user;
mod checkbot;
mod util;

async fn serve_web(address: String, pool: db::PgPool) -> std::io::Result<()> {
    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(util::bearer_auth::bearer_validator);
        App::new()
            .wrap(Cors::default())
            .wrap(auth)
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/v1/users").configure(user::handler::route))
            .service(web::scope("/v1/checkbots")).configure(checkbot::handler::route)
    })
    .bind(address)?
    .run()
    .await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = &env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = &env::var("PORT").unwrap_or("8080".to_string());
    let address = format!("{}:{}", host, port);
    println!("Server running on: {}", address);

    let pool = db::connect_pool();

    serve_web(address, pool).await
}
