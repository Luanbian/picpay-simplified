use crate::services::neo4rs::get_db;
use neo4rs::{Error, query};

pub async fn create_shopman() -> Result<(), Error> {
    let db = get_db().await.unwrap();

    db.run(
        query("CREATE (s:Shopman {name: $name, age: $age})")
            .param("name", "John Doe")
            .param("age", 30),
    )
    .await?;

    Ok(())
}

pub async fn get_shopman() -> Result<(), Error> {
    let db = get_db().await.unwrap();

    let mut result = db
        .execute(query(
            "MATCH (s:Shopman) RETURN s.name AS name, s.age AS age",
        ))
        .await?;

    while let Ok(Some(row)) = result.next().await {
        let name: String = row.get("name").unwrap();
        let age: i64 = row.get("age").unwrap();
        println!("Shopman: name={}, age={}", name, age);
    }

    Ok(())
}
