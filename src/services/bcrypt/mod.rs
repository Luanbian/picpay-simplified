use bcrypt::{DEFAULT_COST, hash};

pub fn encrypt(value: &str) -> String {
    hash(value, DEFAULT_COST).unwrap()
}
