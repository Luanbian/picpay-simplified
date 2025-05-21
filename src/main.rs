mod services;
use axum::Router;
use services::axum::server;
mod constants;
use constants::load_env;
mod features;
use features::shopman::router as shopman_feature;

#[tokio::main]
async fn main() {
    load_env();

    let app = Router::new().nest("/api", Router::new().merge(shopman_feature()));
    server(app).await;
}
