use crate::{
    features::{
        consumer::model::types::ConsumerSchema,
        deposit::{middleware::check_is_consumer, model},
    },
    services::axum::types::ApiResponse,
};
use axum::{
    Json, Router, extract::Path, http::StatusCode, middleware, response::IntoResponse,
    routing::post,
};
use uuid::Uuid;

use super::types::CreateDepositPayload;

async fn deposit(
    Path(id): Path<String>,
    Json(payload): Json<CreateDepositPayload>,
) -> impl IntoResponse {
    let deposit = model::types::DepositSchema {
        id: id.clone(),
        value: (payload.value * 100.0).trunc() as i64,
    };

    let result = model::create_deposit(deposit).await;

    match result {
        Ok(consumer) => {
            let response: ApiResponse<ConsumerSchema, String> = ApiResponse {
                code: String::from("features.deposit.create"),
                transaction: Uuid::new_v4().to_string(),
                message: String::from("Deposit created successfully"),
                args: None,
                data: Some(consumer),
            };
            (StatusCode::OK, Json(response))
        }
        Err(err) => {
            let response: ApiResponse<ConsumerSchema, String> = ApiResponse {
                code: String::from("features.deposit.create"),
                transaction: Uuid::new_v4().to_string(),
                message: String::from("Deposit create failed"),
                args: Some(err.to_string()),
                data: None,
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

pub fn router() -> Router {
    Router::new()
        .route("/{id}", post(deposit))
        .route_layer(middleware::from_fn(check_is_consumer))
}
