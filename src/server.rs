use crate::db;
use crate::students;
use crate::middleware;
use crate::users;
use crate::subscriptions;
use crate::prompts;
use crate::checkbots;
use crate::translation;
use crate::stt;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web, App, HttpServer};
use std::env;
use crate::schema::speech_to_text::dsl::speech_to_text;

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

    // TODO: Remove subscriptions, students, and prompts
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
                .service(web::scope("/v1/checkbots").configure(checkbots::services))
                .service(web::scope("/v1/translation").configure(translation::services))
                .service(web::scope("/v1/speech-to-text").configure(stt::services))
        })
        .bind(Self::address())?
        .run()
        .await
    }
}
