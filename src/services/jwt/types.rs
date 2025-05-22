use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JWTClaims {
    pub id: String,
    pub user: String,
}
