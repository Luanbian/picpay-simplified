use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConsumerSchema {
    pub id: String,
    pub name: String,
    pub cpf: String,
    pub email: String,
    pub password: String,
    pub balance: Option<i64>,
    pub user: String,
}
