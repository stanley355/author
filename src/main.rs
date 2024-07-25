// Cargo
use dotenv::dotenv;

// Modules
mod db;
mod http_error;
mod middleware;
mod schema;
mod server;
mod students;
mod subscriptions;
mod users;
mod v2;

// Flatten
use server::Server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = db::connect_pool();
    Server::new_http_server(pool).await
}
