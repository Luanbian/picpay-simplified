pub fn get_neo4j_uri() -> String {
    std::env::var("NEO4J_URI").unwrap_or_else(|_| "neo4j_uri".to_string())
}

pub fn get_neo4j_username() -> String {
    std::env::var("NEO4J_USERNAME").unwrap_or_else(|_| "neo4j_username".to_string())
}

pub fn get_neo4j_password() -> String {
    std::env::var("NEO4J_PASSWORD").unwrap_or_else(|_| "neo4j_password".to_string())
}
