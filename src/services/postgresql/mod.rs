use crate::constants::postgresql::get_postgresql_url;
use sqlx::{Error, PgPool};
use std::sync::Arc;
use tokio::sync::OnceCell;

static DB: OnceCell<Arc<PgPool>> = OnceCell::const_new();

pub async fn init_db() -> Result<(), Error> {
    let url = get_postgresql_url();

    match PgPool::connect(&url).await {
        Ok(conn) => {
            println!("Connected to PostgreSQL database");
            DB.set(Arc::new(conn)).ok();
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to connect to PostgreSQL database: {e}");
            Err(e)
        }
    }
}

pub async fn get_db() -> Option<Arc<PgPool>> {
    DB.get().cloned()
}
