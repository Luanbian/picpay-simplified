use jsonwebtoken::{EncodingKey, Header, encode};

use crate::constants::jwt::get_jwt_secret;

#[allow(dead_code)]
pub fn sign_jwt(payload: String) -> String {
    encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(get_jwt_secret().as_bytes()),
    )
    .unwrap()
}
