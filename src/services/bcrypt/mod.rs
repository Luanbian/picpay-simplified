use bcrypt::{DEFAULT_COST, hash, verify};

pub fn encrypt(value: &str) -> String {
    hash(value, DEFAULT_COST).unwrap()
}

pub fn verify_hash(password: &str, hash: &str) -> bool {
    verify(password, hash).unwrap_or(false)
}
