use axum::Router;

pub mod core;
pub mod transaction;
pub mod types;

pub fn router() -> Router {
    Router::new().nest(
        "/consumer",
        Router::new()
            .merge(core::router())
            .merge(transaction::router()),
    )
}
