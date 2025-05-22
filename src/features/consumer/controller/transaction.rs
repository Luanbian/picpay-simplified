use crate::features::consumer::{model, model::types::TransactionConsumerSchema};
use crate::features::shopman::model as shopman_model;
use crate::services::axum::types::ApiResponse;
use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::post};
use chrono::Utc;
use uuid::Uuid;

use super::types::TransactionPayload;

async fn transaction(Json(payload): Json<TransactionPayload>) -> impl IntoResponse {
    let shopman = match shopman_model::find_shopman_by_id(payload.to).await {
        Some(shopman) => shopman,
        None => {
            let response: ApiResponse<TransactionConsumerSchema, String> = ApiResponse {
                code: String::from("features.consumer.transaction"),
                transaction: Uuid::new_v4().to_string(),
                message: String::from("Transaction customer failed"),
                data: None::<TransactionConsumerSchema>,
                args: Some(String::from("Shopman not found")),
            };
            return (StatusCode::NOT_FOUND, Json(response));
        }
    };

    let transaction = TransactionConsumerSchema {
        id: Uuid::new_v4().to_string(),
        from: payload.from,
        to: shopman.id,
        amount: (payload.amount * 100.0) as i64,
        when: Utc::now().to_rfc3339().to_string(),
    };

    let result = model::create_transaction(transaction).await;

    match result {
        Ok(ts) => {
            let response: ApiResponse<TransactionConsumerSchema, String> = ApiResponse {
                code: String::from("features.consumer.transaction"),
                transaction: Uuid::new_v4().to_string(),
                message: String::from("Transaction made successfully"),
                data: Some(ts),
                args: None::<String>,
            };
            (StatusCode::OK, Json(response))
        }
        Err(err) => {
            let response: ApiResponse<TransactionConsumerSchema, String> = ApiResponse {
                code: String::from("features.consumer.transaction"),
                transaction: Uuid::new_v4().to_string(),
                message: String::from("Transaction customer failed"),
                data: None::<TransactionConsumerSchema>,
                args: Some(err.to_string()),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

pub fn router() -> Router {
    Router::new().route("/transaction", post(transaction))
}
