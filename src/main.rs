mod framework;

use framework::server::server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server().await
}
