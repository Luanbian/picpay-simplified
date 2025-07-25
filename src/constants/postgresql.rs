pub fn get_postgresql_url() -> String {
    std::env::var("POSTGRESQL_URL").unwrap_or_else(|_| "postgresql_url".to_string())
}
