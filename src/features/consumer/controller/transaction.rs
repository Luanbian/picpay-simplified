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

    let amount = (payload.amount * 100.0) as i64;
    let has_sufficient_balance = match model::validate_balance(payload.from.clone(), amount).await {
        Ok(balance) => balance,
        Err(err) => {
            let response: ApiResponse<TransactionConsumerSchema, String> = ApiResponse {
                code: String::from("features.consumer.transaction"),
                transaction: Uuid::new_v4().to_string(),
                message: String::from("Verify balance failed"),
                data: None::<TransactionConsumerSchema>,
                args: Some(err.to_string()),
            };
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
        }
    };

    if !has_sufficient_balance {
        let response: ApiResponse<TransactionConsumerSchema, String> = ApiResponse {
            code: String::from("features.consumer.transaction"),
            transaction: Uuid::new_v4().to_string(),
            message: String::from("Transaction customer failed"),
            data: None::<TransactionConsumerSchema>,
            args: Some(String::from("Insufficient balance")),
        };
        return (StatusCode::BAD_REQUEST, Json(response));
    }

    let transaction = TransactionConsumerSchema {
        id: Uuid::new_v4().to_string(),
        from: payload.from,
        to: shopman.id,
        amount,
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
