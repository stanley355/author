// Cargo 
use dotenv::dotenv;

// Modules
mod db;
mod schema;
mod server;
mod v2;
mod users;
mod middleware;

// Flatten
use server::Server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = db::connect_pool();
    Server::new_http_server(pool).await
}
