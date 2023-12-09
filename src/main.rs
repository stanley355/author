#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use dotenv::dotenv;
use std::env;
use util::bearer;

mod db;
mod prompt;
mod schema;
mod subscription;
mod topup;
mod user;
mod util;

async fn serve_web(address: String, pool: db::PgPool) -> std::io::Result<()> {

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default())
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/v1/users").configure(user::handler::route))
            .service(web::scope("/v1/prompts").configure(prompt::handler::route))
            .service(web::scope("/v1/topups").configure(topup::handler::route))
            .service(web::scope("/v1/subscriptions").configure(subscription::handler::route))
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
