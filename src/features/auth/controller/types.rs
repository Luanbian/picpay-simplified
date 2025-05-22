use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}
