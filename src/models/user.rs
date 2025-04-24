use actix_web::web;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tokio_postgres::Client;

use crate::controllers::{send_otp::OtpEmailRequest, signup_user::UserData};

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
    pub otp: i16,
    pub device_id: String,
}

impl User {
    pub fn validate(data: &UserData) -> Result<(), String> {
        if data.device_id.is_empty() || data.device_id.len() < 10 || data.device_id.len() > 50 {
            return Err("Device ID must be in proper formate.".to_string());
        }
        if data.otp < 1000 || data.otp > 9999 {
            return Err("Improper OTP".to_string());
        }
        if data.email.is_empty() {
            return Err("Email is required".to_string());
        }
        if !data.email.contains('@') {
            return Err("Invalid email format".to_string());
        }
        if data.password_hash.is_empty() {
            return Err("Password is required".to_string());
        }
        let min_length = 8;
        if data.password_hash.len() < min_length {
            return Err(format!(
                "Password must be at least {} characters long",
                min_length
            ));
        }

        if !UPPER_CASE_REGEX.is_match(&data.password_hash) {
            return Err("Password must contain at least one uppercase letter".to_string());
        }

        if !LOWER_CASE_REGEX.is_match(&data.password_hash) {
            return Err("Password must contain at least one lowercase letter".to_string());
        }

        if !NUMBER_REGEX.is_match(&data.password_hash) {
            return Err("Password must contain at least one number".to_string());
        }

        Ok(())
    }
}
