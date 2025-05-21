use axum::Router;

pub mod core;

pub fn router() -> Router {
    Router::new().nest("/shopman", core::router())
}
