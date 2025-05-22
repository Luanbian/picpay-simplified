use crate::services::axum::types::ApiResponse;
use crate::{features::consumer::model, services::bcrypt::encrypt};
use axum::{
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use uuid::Uuid;

use super::types::CreateConsumerPayload;

async fn create_consumer(Json(payload): Json<CreateConsumerPayload>) -> impl IntoResponse {
    let consumer = model::types::ConsumerSchema {
        id: Uuid::new_v4().to_string(),
        name: payload.name,
        cpf: payload.cpf,
        email: payload.email,
        password: encrypt(&payload.password),
        balance: None,
    };

    let result = model::create_consumer(consumer).await;

    match result {
        Ok(id) => {
            let response: ApiResponse<String, String> = ApiResponse {
                code: String::from("features.consumer.create"),
                transaction: Uuid::new_v4().to_string(),
                message: String::from("Consumer created successfully"),
                args: None,
                data: Some(id),
            };
            (StatusCode::CREATED, Json(response))
        }
        Err(err) => {
            let response: ApiResponse<String, String> = ApiResponse {
                code: String::from("features.consumer.create"),
                transaction: Uuid::new_v4().to_string(),
                message: String::from("Consumer create failed"),
                args: Some(err.to_string()),
                data: None,
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

async fn find_consumers() -> impl IntoResponse {
    let result = model::get_consumers().await;

    match result {
        Ok(consumers) => {
            let response: ApiResponse<Vec<model::types::ConsumerSchema>, String> = ApiResponse {
                code: String::from("features.consumer.find"),
                transaction: Uuid::new_v4().to_string(),
                message: String::from("Consumers reader successfully"),
                args: None,
                data: Some(consumers),
            };
            (StatusCode::OK, Json(response))
        }
        Err(err) => {
            let response: ApiResponse<Vec<model::types::ConsumerSchema>, String> = ApiResponse {
                code: String::from("features.consumer.find"),
                transaction: Uuid::new_v4().to_string(),
                message: String::from("Consumers reader failed"),
                args: Some(err.to_string()),
                data: None,
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

pub fn router() -> Router {
    Router::new()
        .route("/", post(create_consumer))
        .route("/", get(find_consumers))
}
