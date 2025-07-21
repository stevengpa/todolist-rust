mod framework;
mod domain;

use std::env;

use dotenvy::dotenv;
use framework::server::server;
use crate::framework::config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /* Run commands
    cargo install cargo-watch
    cargo watch -w src -w templates -x 'run'
    */
    dotenv().ok();
    let envs: Config = envy::from_env().expect("Failed to load config");
    // let host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    // let port = env::var("APP_PORT").unwrap_or_else(|_| "8000".to_string()).parse::<u16>().expect("PORT mut be a number");

    server(envs).await
}
