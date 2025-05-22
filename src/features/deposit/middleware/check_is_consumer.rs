use axum::{
    Json,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde_json::json;

use crate::services::jwt::verify_jwt;

pub async fn check_is_consumer(req: Request, next: Next) -> Result<Response, Response> {
    let auth = req
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok());

    let token = match auth {
        Some(token) => &token[7..],
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "No token provided"})),
            )
                .into_response());
        }
    };

    let user = verify_jwt(token);
    match user {
        Ok(user) => {
            if user.user == "consumer" {
                Ok(next.run(req).await)
            } else {
                Err((
                    StatusCode::FORBIDDEN,
                    Json(json!({"message": "You are not allowed to do this action"})),
                )
                    .into_response())
            }
        }
        Err(_) => Err((
            StatusCode::FORBIDDEN,
            Json(json!({"message": "Invalid Token"})),
        )
            .into_response()),
    }
}
