use crate::services::neo4rs::get_db;
use futures::TryStreamExt;
use neo4rs::{Error, Node, query};

use super::types::ShopmanSchema;

pub async fn create_shopman(shopman: ShopmanSchema) -> Result<String, Error> {
    let db = get_db().await.unwrap();

    db.run(
        query("CREATE (s:Shopman {id: $id, company: $company, cnpj: $cnpj, email: $email, password: $password, balance: $balance, user: $user})")
            .param("id", shopman.id.clone())
            .param("company", shopman.company)
            .param("cnpj", shopman.cnpj)
            .param("email", shopman.email)
            .param("password", shopman.password)
            .param("balance", shopman.balance)
            .param("user", shopman.user),
    )
    .await?;

    Ok(shopman.id)
}

pub async fn get_shopman() -> Result<Vec<ShopmanSchema>, Error> {
    let db = get_db().await.unwrap();

    let result = db.execute(query("MATCH (s:Shopman) RETURN s")).await?;

    let shopmen = result
        .into_stream()
        .map_ok(|row| {
            let node: Node = row.get("s").unwrap();

            ShopmanSchema {
                id: node.get("id").unwrap(),
                company: node.get("company").unwrap(),
                cnpj: node.get("cnpj").unwrap(),
                email: node.get("email").unwrap(),
                password: node.get("password").unwrap(),
                balance: node.get("balance").unwrap_or(None),
                user: node.get("user").unwrap(),
            }
        })
        .try_collect()
        .await?;

    Ok(shopmen)
}
