use dotenv::dotenv;

pub mod axum;
pub mod jwt;
pub mod neo4j;

pub fn load_env() {
    dotenv().ok();
}
