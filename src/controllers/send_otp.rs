use actix_web::{web, HttpResponse, Responder};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::utils::{record_handler::RecordHandler, user_authentication::UserAuthentication};

#[derive(Deserialize, Serialize)]
pub struct OtpEmailRequest {
    pub email: String,
}

pub async fn send_otp(data: web::Json<OtpEmailRequest>) -> impl Responder {
    let otp: i16 = rand::thread_rng().gen_range(1000..10000);

    let response = UserAuthentication::send_otp_to_email(&data, otp).await;

    match response {
        Ok(_) => {
            let table: String = "otp".to_string();
            let creation_result = RecordHandler::create_data(
                table, // This is your table name
                json!( {
                    "email": data.email.to_string(),
                    "otp": otp
                }),
            )
            .await;

            match creation_result {
                Ok(_) => HttpResponse::Ok().body("OTP sent and stored successfully!"),

                Err(e) => HttpResponse::InternalServerError()
                    .body(format!("OTP sent, but failed to store: {}", e)),
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to send email: {}", e)),
    }
}
