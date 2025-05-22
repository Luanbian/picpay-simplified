use crate::services::axum::types::ApiResponse;
use crate::{features::shopman::model, services::bcrypt::encrypt};
use axum::{
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use uuid::Uuid;

use super::types::CreateShopmanPayload;

async fn create_shopman(Json(payload): Json<CreateShopmanPayload>) -> impl IntoResponse {
    let shopman = model::types::ShopmanSchema {
        id: Uuid::new_v4().to_string(),
        company: payload.company,
        cnpj: payload.cnpj,
        email: payload.email,
        password: encrypt(&payload.password),
        balance: None,
        user: String::from("shopman"),
    };

    let result = model::create_shopman(shopman).await;

    match result {
        Ok(id) => {
            let response: ApiResponse<String, String> = ApiResponse {
                code: String::from("features.shopman.create"),
                transaction: Uuid::new_v4().to_string(),
                message: String::from("Shopman created successfully"),
                args: None,
                data: Some(id),
            };
            (StatusCode::CREATED, Json(response))
        }
        Err(err) => {
            let response: ApiResponse<String, String> = ApiResponse {
                code: String::from("features.shopman.create"),
                transaction: Uuid::new_v4().to_string(),
                message: String::from("Shopman create failed"),
                args: Some(err.to_string()),
                data: None,
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

async fn find_shopmen() -> impl IntoResponse {
    let result = model::get_shopman().await;

    match result {
        Ok(shopmen) => {
            let response: ApiResponse<Vec<model::types::ShopmanSchema>, String> = ApiResponse {
                code: String::from("features.shopman.find"),
                transaction: Uuid::new_v4().to_string(),
                message: String::from("Shopmen reader successfully"),
                args: None,
                data: Some(shopmen),
            };
            (StatusCode::OK, Json(response))
        }
        Err(err) => {
            let response: ApiResponse<Vec<model::types::ShopmanSchema>, String> = ApiResponse {
                code: String::from("features.shopman.find"),
                transaction: Uuid::new_v4().to_string(),
                message: String::from("Shopmen reader failed"),
                args: Some(err.to_string()),
                data: None,
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

pub fn router() -> Router {
    Router::new()
        .route("/", post(create_shopman))
        .route("/", get(find_shopmen))
}
