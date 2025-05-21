use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ShopmanSchema {
    pub id: String,
    pub company: String,
    pub cnpj: String,
    pub email: String,
    pub password: String,
}
