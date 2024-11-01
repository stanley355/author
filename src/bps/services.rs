use actix_web::web;
use super::dynamic_data;

pub fn services(config: &mut web::ServiceConfig) {
    config.service(web::scope("/dynamic-data").configure(dynamic_data::services));
}
