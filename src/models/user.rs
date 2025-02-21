use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref UPPER_CASE_REGEX: Regex = Regex::new(r"[A-Z]").unwrap();
    static ref LOWER_CASE_REGEX: Regex = Regex::new(r"[a-z]").unwrap();
    static ref NUMBER_REGEX: Regex = Regex::new(r"\d").unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)] // Ensure no extra fields are allowed
pub struct User {
    pub email: String,
    pub password_hash: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
}

impl User {
    pub fn validate(&self) -> Result<(), String> {
        if self.email.is_empty() {
            return Err("Email is required".to_string());
        }
        if !self.email.contains('@') {
            return Err("Invalid email format".to_string());
        }
        if self.password_hash.is_empty() {
            return Err("Password is required".to_string());
        }
        let min_length = 8;
        if self.password_hash.len() < min_length {
            return Err(format!(
                "Password must be at least {} characters long",
                min_length
            ));
        }

        if !UPPER_CASE_REGEX.is_match(&self.password_hash) {
            return Err("Password must contain at least one uppercase letter".to_string());
        }

        if !LOWER_CASE_REGEX.is_match(&self.password_hash) {
            return Err("Password must contain at least one lowercase letter".to_string());
        }

        if !NUMBER_REGEX.is_match(&self.password_hash) {
            return Err("Password must contain at least one number".to_string());
        }

        Ok(())
    }
}
