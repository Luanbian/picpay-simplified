use crate::constants::neo4j::{get_neo4j_password, get_neo4j_uri, get_neo4j_username};
use neo4rs::{ConfigBuilder, Error, Graph};
use std::sync::Arc;
use tokio::sync::OnceCell;

static DB: OnceCell<Arc<Graph>> = OnceCell::const_new();

pub async fn init_db() -> Result<(), Error> {
    let config = ConfigBuilder::default()
        .uri(get_neo4j_uri())
        .user(get_neo4j_username())
        .password(get_neo4j_password())
        .build()
        .unwrap();

    match Graph::connect(config).await {
        Ok(graph) => {
            println!("Connected to Neo4j database");
            DB.set(Arc::new(graph)).ok();
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to connect to Neo4j database: {:?}", e);
            Err(e)
        }
    }
}

pub async fn get_db() -> Option<Arc<Graph>> {
    DB.get().cloned()
}
