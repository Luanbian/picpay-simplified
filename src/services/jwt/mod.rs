use crate::constants::jwt::get_jwt_secret;
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode, errors::Error};

pub mod types;
use types::JWTClaims;

pub fn sign_jwt(id: &str, user: &str) -> String {
    let claims = JWTClaims {
        id: id.to_string(),
        user: user.to_string(),
        exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_jwt_secret().as_bytes()),
    )
    .unwrap()
}

pub fn verify_jwt(token: &str) -> Result<JWTClaims, Error> {
    let token_data = jsonwebtoken::decode::<JWTClaims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(get_jwt_secret().as_bytes()),
        &jsonwebtoken::Validation::default(),
    )?;
    Ok(token_data.claims)
}
