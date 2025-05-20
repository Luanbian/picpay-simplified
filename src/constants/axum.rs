pub fn get_axum_port() -> u16 {
    std::env::var("AXUM_PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .unwrap_or(3000)
}

