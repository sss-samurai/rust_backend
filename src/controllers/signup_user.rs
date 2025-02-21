// src/handlers/signup_user.rs

use crate::models::user::User;
use crate::utils::record_handler::RecordHandler;
use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

pub async fn signup_user(json_data: web::Json<Value>) -> impl Responder {
    match serde_json::from_value::<User>(json_data.into_inner()) {
        Ok(user) => {
            if let Err(validation_error) = user.validate() {
                return HttpResponse::BadRequest().json(ErrorResponse {
                    error: validation_error,
                });
            }

            // return HttpResponse::Ok().json(json!({
            //     "message": "Validation successful, but stopping here for experiment",
            //     "data": user
            // }));

            match RecordHandler::create_data("users", json!(user)).await {
                Ok(created_user) => HttpResponse::Ok().json(json!({
                    "message": "User created successfully",
                    "data": created_user
                })),
                Err(e) => HttpResponse::InternalServerError().json(json!({
                    "message": "Failed to create user",
                    "error": e.to_string()
                })),
            }
        }
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse {
            error: format!("Invalid user data provided. Error: {}", e),
        }),
    }
}
