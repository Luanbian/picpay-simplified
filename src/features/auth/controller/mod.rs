use axum::Router;

pub mod core;
pub mod types;

pub fn router() -> Router {
    Router::new().nest("/auth", core::router())
}
