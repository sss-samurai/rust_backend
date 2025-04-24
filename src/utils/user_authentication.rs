use crate::controllers::{send_otp::OtpEmailRequest, signup_user::UserData};
use actix_web::web;
use anyhow::{Context, Error as AnyhowError};
use lettre::{
    message::SinglePart, transport::smtp::authentication::Credentials, Message, SmtpTransport,
    Transport,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::env;
use tokio_postgres::NoTls;
#[derive(Debug, Serialize)]
pub struct EmailRequest {
    pub email: String,
}

pub struct UserAuthentication;

impl UserAuthentication {
    pub async fn send_otp_to_email(
        data: &web::Json<OtpEmailRequest>,
        otp: i16,
    ) -> Result<(), String> {
        let sender_email = "sunindrasingh91@gmail.com";
        let recipient_email = data.email.to_string();
        let email_result = tokio::task::spawn_blocking(move || {
            let sender = match sender_email.parse() {
                Ok(email) => email,
                Err(e) => return Err(format!("Failed to parse sender email: {}", e)),
            };
            let recipient = match recipient_email.parse() {
                Ok(email) => email,
                Err(e) => return Err(format!("Failed to parse recipient email: {}", e)),
            };
            let email = Message::builder()
                .from(sender)
                .to(recipient)
                .subject(format!("Test Email from Rust {}", otp))
                .singlepart(SinglePart::plain(
                    "This is a test email sent from Rust!".to_string(),
                ))
                .map_err(|e| format!("Failed to create email body: {}", e))?;
            let app_password = "bzdm cpzo jklt ecbg";
            let creds = Credentials::new(sender_email.to_string(), app_password.to_string());
            let mailer = SmtpTransport::relay("smtp.gmail.com")
                .map_err(|e| format!("Failed to configure SMTP relay: {}", e))?
                .credentials(creds)
                .build();
            mailer
                .send(&email)
                .map_err(|e| format!("Failed to send email: {}", e))
        })
        .await;
        match email_result {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(e) => Err(format!("Task execution failed: {}", e)),
        }
    }

    pub async fn check_match(
        table: &str,
        columns: Vec<String>,
        values: Vec<String>,
    ) -> Result<bool, AnyhowError> {
        if columns.len() != values.len() {
            return Err(AnyhowError::msg(
                "Number of columns does not match number of values",
            ));
        }
        let database_url =
            env::var("DATABASE_URL").context("DATABASE_URL environment variable not set")?;
        let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
            .await
            .context("Failed to connect to the database")?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });
        let columns_str = columns.join(", ");
        let query = format!(
            "SELECT EXISTS (SELECT 1 FROM {} WHERE {} LIMIT 1)",
            table, columns_str
        );
        let row = client
            .query_one(
                query.as_str(),
                &values
                    .iter()
                    .map(|v| v as &(dyn tokio_postgres::types::ToSql + Sync))
                    .collect::<Vec<_>>(),
            )
            .await
            .context("Failed to execute query")?;

        let exists: bool = row.get(0);
        Ok(exists)
    }

    pub fn generate_key(user_data: &UserData) -> String {
        let data = [
            &user_data.email,
            &user_data.password_hash,
            user_data.first_name.as_deref().unwrap_or(""),
            user_data.last_name.as_deref().unwrap_or(""),
            user_data.phone_number.as_deref().unwrap_or(""),
            &user_data.otp.to_string(),
            &user_data.device_id,
        ]
        .concat();
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        result
            .iter()
            .fold(String::new(), |acc, byte| acc + &format!("{:02x}", byte))
    }
}
