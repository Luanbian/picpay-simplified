use super::types::DepositSchema;
use crate::{features::consumer::model::types::ConsumerSchema, services::neo4rs::get_db};
use neo4rs::{Error, Node, query};

pub async fn create_deposit(deposit: DepositSchema) -> Result<ConsumerSchema, Error> {
    let db = get_db().await.unwrap();

    let mut result = db
        .execute(
            query("MATCH (c:Consumer {id: $id}) SET c.balance = $value RETURN c")
                .param("id", deposit.id)
                .param("value", deposit.value),
        )
        .await?;

    if let Some(row) = result.next().await? {
        let node: Node = row.get("c").unwrap();

        let consumer = ConsumerSchema {
            id: node.get("id").unwrap(),
            name: node.get("name").unwrap(),
            cpf: node.get("cpf").unwrap(),
            email: node.get("email").unwrap(),
            password: node.get("password").unwrap(),
            balance: node.get("balance").unwrap_or(None),
            user: node.get("user").unwrap(),
        };
        Ok(consumer)
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "No register found").into())
    }
}
