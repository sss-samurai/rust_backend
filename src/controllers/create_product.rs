// src/controllers/create_product.rs
use crate::utils::create_data::create_data;
use actix_web::{web, HttpResponse, Responder};
use serde_json::{json, Value};

pub async fn create_product(json_data: web::Json<Value>) -> impl Responder {
    match create_data("product", json_data.into_inner()).await {
        Ok(created_product) => HttpResponse::Ok().json(json!({
            "message": "Product created successfully",
            "data": created_product
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "message": "Failed to create product",
            "error": e.to_string()
        })),
    }
}
