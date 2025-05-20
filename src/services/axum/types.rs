use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ApiResponse<T, E> {
    pub code: String,
    pub transaction: String,
    pub message: String,
    pub data: Option<T>,
    pub args: Option<E>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct APIEcho {
    pub server: String,
    pub time: String,
    pub version: String,
}
