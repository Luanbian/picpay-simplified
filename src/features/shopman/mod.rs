use axum::{Router, response::IntoResponse, routing::get};

async fn shopman_index() -> impl IntoResponse {
    "Shopman route funcionando!"
}

pub fn router() -> Router {
    Router::new().route("/shopman", get(shopman_index))
}
