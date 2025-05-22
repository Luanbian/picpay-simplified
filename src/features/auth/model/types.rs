use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginSchema {
    pub id: String,
    pub email: String,
    pub password: String,
    pub user: String,
}
