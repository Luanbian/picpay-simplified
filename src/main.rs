mod services;
use services::axum::server;

#[tokio::main]
async fn main() {
    server().await;
}
