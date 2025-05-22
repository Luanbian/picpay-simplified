use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateConsumerPayload {
    pub name: String,
    pub cpf: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionPayload {
    pub from: String,
    pub to: String,
    pub amount: f64,
}
