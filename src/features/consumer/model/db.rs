use crate::services::neo4rs::get_db;
use futures::TryStreamExt;
use neo4rs::{Error, Node, query};

use super::types::{ConsumerSchema, TransactionConsumerSchema};

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

pub async fn create_transaction(
    transaction: TransactionConsumerSchema,
) -> Result<TransactionConsumerSchema, Error> {
    let db = get_db().await.unwrap();

    db.run(
        query(
            "MATCH (c:Consumer {id: $from}), (s:Shopman {id: $to}) \
            CREATE (c)-[:CUSTOMER_TRANSACTION {id: $id, amount: $amount, when: $when}]->(s) \
            SET s.balance = coalesce(s.balance, 0) + $amount \
            SET c.balance = coalesce(c.balance, 0) - $amount",
        )
        .param("from", transaction.from.clone())
        .param("to", transaction.to.clone())
        .param("id", transaction.id.clone())
        .param("amount", transaction.amount)
        .param("when", transaction.when.clone()),
    )
    .await?;

    Ok(transaction)
}

pub async fn validate_balance(id: String, amount: i64) -> Result<bool, Error> {
    let db = get_db().await.unwrap();

    let result = db
        .execute(query("MATCH (s:Consumer {id: $id}) RETURN s.balance").param("id", id))
        .await?;

    let balances: Vec<i64> = result
        .into_stream()
        .map_ok(|row| row.get::<i64>("s.balance").unwrap_or(0))
        .try_collect()
        .await?;

    Ok(balances.first().is_some_and(|&balance| balance >= amount))
}
