// Cargo
use dotenv::dotenv;

// Modules
mod db;
mod http_error;
mod middleware;
mod schema;
mod server;
mod users;
mod openai;
mod checkbots;
mod translation;
mod stt;
mod tts;
mod bps;

// Flatten
use server::Server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = db::connect_pool();
    Server::new_http_server(pool).await
}
