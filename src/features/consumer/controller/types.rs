use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateConsumerPayload {
    pub name: String,
    pub cpf: String,
    pub email: String,
    pub password: String,
}
