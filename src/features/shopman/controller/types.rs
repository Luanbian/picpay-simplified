use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateShopmanPayload {
    pub company: String,
    pub cnpj: String,
    pub email: String,
    pub password: String,
}
