use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, PasswordHash};
use rand::rngs::OsRng;
use jsonwebtoken::{encode, Header, EncodingKey};
use crate::models::Claims;
use chrono::Utc;


pub fn hash_password(password: &str) -> String {
    let mut rng = OsRng;
    let salt = SaltString::generate(&mut rng);

    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}


pub fn verify_password(hash: &str, password: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash).unwrap();

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}


pub fn create_jwt(user_id: &str) -> String {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());

    let exp = (Utc::now() + chrono::Duration::hours(24)).timestamp() as usize;

    let claims = Claims { sub: user_id.to_string(), exp };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes())).unwrap()
}



// opt utilities
