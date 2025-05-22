use axum::Router;

mod constants;
use constants::load_env;
mod services;
use services::axum::server;
use services::neo4rs::init_db;
mod features;
use features::consumer::controller::router as consumer_feature;
use features::deposit::controller::router as deposit_feature;
use features::shopman::controller::router as shopman_feature;

#[tokio::main]
async fn main() {
    load_env();

    init_db().await.unwrap();

    let app = Router::new().nest(
        "/api",
        Router::new()
            .merge(shopman_feature())
            .merge(consumer_feature())
            .merge(deposit_feature()),
    );
    server(app).await;
}
