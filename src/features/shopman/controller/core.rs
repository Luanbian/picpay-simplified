use axum::{Router, response::IntoResponse, routing::post};

async fn shopman_create() -> impl IntoResponse {
    println!("Shopman create");
}

pub fn router() -> Router {
    Router::new().route("/", post(shopman_create))
}
