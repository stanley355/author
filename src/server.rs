use crate::db;
use crate::students;
use crate::middleware;
use crate::users;
use crate::subscriptions;
use crate::prompts;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web, App, HttpServer};
use std::env;

#[derive(Debug)]
pub struct Server;

impl Server {
    fn address() -> String {
        let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
        let port = env::var("PORT").unwrap_or("8080".to_string());
        let address = format!("{}:{}", host, port);
        println!("Server running on: {}", address);
        return address;
    }

    pub(super) async fn new_http_server(pool: db::PgPool) -> std::io::Result<()> {
        HttpServer::new(move || {
            App::new()
                .wrap(Cors::default())
                .wrap(middleware::AuthorMiddleware)
                .app_data(web::Data::new(pool.clone()))
                .service(Files::new("/v1/files", "/tmp"))
                .service(web::scope("/v1/users").configure(users::services))
                .service(web::scope("/v1/subscriptions").configure(subscriptions::services))
                .service(web::scope("/v1/students").configure(students::services))
                .service(web::scope("/v1/prompts").configure(prompts::services))
        })
        .bind(Self::address())?
        .run()
        .await
    }
}
