mod services;
use services::axum::server;
mod constants;
use constants::load_env;

#[tokio::main]
async fn main() {
    load_env();
    server().await;
}
