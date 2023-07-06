use std::ops::Add;
use serde::{Serialize, Deserialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

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
            aud: String::default(),
            exp: SystemTime::now().add(Duration::new(exp, 0)).duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs() as usize,
            iat: SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs() as usize,
            iss: String::default(),
            sub,
        }
    }
}
