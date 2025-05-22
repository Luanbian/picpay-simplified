use crate::{
    features::auth::model,
    services::{axum::types::ApiResponse, bcrypt::verify_hash},
};
use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::post};
use uuid::Uuid;

use super::types::LoginPayload;

async fn login(Json(payload): Json<LoginPayload>) -> impl IntoResponse {
    let user = match model::find_user_by_email(&payload.email).await {
        Some(user) => user,
        None => {
            let response: ApiResponse<String, String> = ApiResponse {
                code: String::from("features:auth:login"),
                transaction: Uuid::new_v4().to_string(),
                message: String::from("Login Failed"),
                args: Some(String::from("User not found")),
                data: None,
            };
            return (StatusCode::NOT_FOUND, Json(response));
        }
    };

    let is_valid = verify_hash(&payload.password, &user.password);
    if !is_valid {
        let response: ApiResponse<String, String> = ApiResponse {
            code: String::from("features:auth:login"),
            transaction: Uuid::new_v4().to_string(),
            message: String::from("Login Failed"),
            args: Some(String::from("Invalid password")),
            data: None,
        };
        return (StatusCode::UNAUTHORIZED, Json(response));
    }

    let response: ApiResponse<String, String> = ApiResponse {
        code: String::from("features:auth:login"),
        transaction: Uuid::new_v4().to_string(),
        message: String::from("Login Successfully"),
        args: None,
        data: Some(String::from("token")),
    };
    (StatusCode::OK, Json(response))
}

pub fn router() -> Router {
    Router::new().route("/login", post(login))
}
