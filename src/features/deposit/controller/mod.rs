use axum::Router;

pub mod core;
pub mod types;

pub fn router() -> Router {
    Router::new().nest("/deposit", core::router())
}
