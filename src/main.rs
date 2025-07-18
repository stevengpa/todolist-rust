mod framework;

use framework::server::server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /* Run commands
    cargo install cargo-watch
    cargo watch -w src -w templates -x 'run'
    */
    server("127.0.0.1", 8000).await
}
