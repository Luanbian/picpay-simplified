use axum::{Json, Router, routing::get};
use chrono::Utc;
use hostname::get as getHost;
use tokio::net::TcpListener;
use uuid::Uuid;

mod types;
use crate::constants::axum::get_axum_port;
use types::{APIEcho, ApiResponse};

pub async fn server(app: Router) {
    let app = health_check(app);
    let listener = listener().await;

    axum::serve(listener, app).await.unwrap();
}

async fn listener() -> TcpListener {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", get_axum_port()))
        .await
        .unwrap();
    println!("Server Listening on port {}", get_axum_port());
    listener
}

fn health_check(app: Router) -> Router {
    app.route(
        "/api",
        get(|| async {
            let response: ApiResponse<APIEcho, ()> = ApiResponse {
                code: String::from("echo"),
                transaction: Uuid::new_v4().to_string(),
                message: String::from("Ok"),
                args: None,
                data: Some(APIEcho {
                    server: getHost()
                        .ok()
                        .and_then(|h| h.into_string().ok())
                        .unwrap_or_else(|| "unknown".to_string()),
                    time: Utc::now().to_rfc3339(),
                    version: env!("CARGO_PKG_VERSION").to_string(),
                }),
            };

            Json(response)
        }),
    )
}
