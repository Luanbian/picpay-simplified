use crate::services::neo4rs::get_db;
use futures::TryStreamExt;
use neo4rs::{Error, Node, query};

use super::types::ConsumerSchema;

pub async fn create_consumer(consumer: ConsumerSchema) -> Result<String, Error> {
    let db = get_db().await.unwrap();

    db.run(
        query("CREATE (s:Consumer {id: $id, name: $name, cpf: $cpf, email: $email, password: $password, balance: $balance, user: $user})")
            .param("id", consumer.id.clone())
            .param("name", consumer.name)
            .param("cpf", consumer.cpf)
            .param("email", consumer.email)
            .param("password", consumer.password)
            .param("balance", consumer.balance)
            .param("user", consumer.user),
    )
    .await?;

    Ok(consumer.id)
}

pub async fn get_consumers() -> Result<Vec<ConsumerSchema>, Error> {
    let db = get_db().await.unwrap();

    let result = db.execute(query("MATCH (s:Consumer) RETURN s")).await?;

    let consumers = result
        .into_stream()
        .map_ok(|row| {
            let node: Node = row.get("s").unwrap();

            ConsumerSchema {
                id: node.get("id").unwrap(),
                name: node.get("name").unwrap(),
                cpf: node.get("cpf").unwrap(),
                email: node.get("email").unwrap(),
                password: node.get("password").unwrap(),
                balance: node.get("balance").unwrap_or(None),
                user: node.get("user").unwrap(),
            }
        })
        .try_collect()
        .await?;

    Ok(consumers)
}
