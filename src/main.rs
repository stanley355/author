#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use tracing_subscriber::prelude::*;

mod db;
mod schema;
mod v2;

async fn serve_web(address: String, pool: db::PgPool) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default())
            .wrap(Logger::default())
            .wrap(v2::middleware::author_middleware::AuthorMiddleware)
            .app_data(web::Data::new(pool.clone()))
            .service(Files::new("/v1/files", "/tmp"))
            .service(web::scope("/v2/prompts").configure(v2::prompt::handler::route))
            .service(web::scope("/v2/users").configure(v2::user::handler::route))
            .service(web::scope("/v2/topups").configure(v2::topup::handler::route))
            .service(web::scope("/v2/students").configure(v2::student::handler::route))
    })
    .bind(address)?
    .run()
    .await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let sentry_key = env::var("SENTRY_KEY".to_string()).unwrap_or_default();
    if sentry_key != "".to_string() {
        tracing_subscriber::Registry::default()
            .with(tracing_subscriber::fmt::layer())
            .with(sentry_tracing::layer())
            .init();
        let _guard = sentry::init((
            sentry_key,
            sentry::ClientOptions {
                release: sentry::release_name!(),
                traces_sample_rate: 1.0,
                ..Default::default()
            },
        ));
    }

    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("8080".to_string());
    let address = format!("{}:{}", host, port);
    println!("Server running on: {}", address);

    let pool = db::connect_pool();
    serve_web(address, pool).await
}
