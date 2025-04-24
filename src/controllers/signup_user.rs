use crate::{models::user::User, utils::user_authentication::UserAuthentication};
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserData {
    pub email: String,
    pub password_hash: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub otp: i16,
    pub device_id: String,
}

pub async fn signup_user(json_data: web::Json<UserData>) -> impl Responder {
    let user_data = json_data.into_inner();

    match User::validate(&user_data) {
        Ok(_) => {
            let columns: Vec<String> = vec!["email".to_string(), "otp".to_string()];
            let values: Vec<String> = vec![user_data.email.to_string(), user_data.otp.to_string()];

            if UserAuthentication::check_match("users", columns, values)
                .await
                .unwrap_or(false)
            {
                let generated_key = UserAuthentication::generate_key(&user_data);

                HttpResponse::Ok().json(json!({
                    "message": "User signed up successfully",
                    "generated_key": generated_key
                }))
            } else {
                HttpResponse::BadRequest().json(ErrorResponse {
                    error: "Invalid OTP".to_string(),
                })
            }
        }
        Err(validation_error) => HttpResponse::BadRequest().json(ErrorResponse {
            error: validation_error,
        }),
    }
}
