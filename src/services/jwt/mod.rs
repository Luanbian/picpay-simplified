use crate::constants::jwt::get_jwt_secret;
use jsonwebtoken::{EncodingKey, Header, encode};
pub mod types;
use types::JWTClaims;

pub fn sign_jwt(id: &str, user: &str) -> String {
    let claims = JWTClaims {
        id: id.to_string(),
        user: user.to_string(),
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_jwt_secret().as_bytes()),
    )
    .unwrap()
}
