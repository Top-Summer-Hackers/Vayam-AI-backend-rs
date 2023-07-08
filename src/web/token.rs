use std::ops::Add;
use serde::{Serialize, Deserialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use jsonwebtoken::{encode, EncodingKey, Header};
use tower_cookies::Cookie;
use crate::web::{AUTH_TOKEN, SECRET};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    aud: String,         // Optional. Audience
    exp: usize,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize,          // Optional. Issued at (as UTC timestamp)
    iss: String,         // Optional. Issuer
    sub: String,         // Optional. Subject (whom token refers to)
}

impl Claims {
    pub fn new(sub: String, exp: u64) -> Claims {
        Claims {
            aud: String::from("http://localhost:8080"),
            exp: SystemTime::now().add(Duration::new(exp, 0)).duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs() as usize,
            iat: SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs() as usize,
            iss: String::default(),
            sub,
        }
    }
}

pub fn generate_token(user_id: String, exp: Option<u64>) -> String {
    encode(
        &Header::default(),
        &Claims::new(user_id, exp.unwrap_or(24 * 3600)),
        &EncodingKey::from_secret(SECRET.as_ref())
    ).expect("Couldn't generate token")
}

/// Generates the auth cookie
/// If `exp` is `None` then it expires in 24h
pub fn generate_auth_cookie(user_id: String, exp: Option<u64>) -> Cookie<'static> {
    Cookie::new(AUTH_TOKEN, generate_token(user_id, exp))
}