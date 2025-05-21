use crate::features::shopman::model;
use axum::{Router, response::IntoResponse, routing::post};

async fn shopman_create() -> impl IntoResponse {
    model::main();
    println!("Shopman create");
}

pub fn router() -> Router {
    Router::new().route("/", post(shopman_create))
}
