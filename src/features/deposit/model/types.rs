use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DepositSchema {
    pub id: String,
    pub value: i64,
}
