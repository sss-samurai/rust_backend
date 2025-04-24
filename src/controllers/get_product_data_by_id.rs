use crate::utils::get_data_by_id::get_data_by_id;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

pub async fn get_product_data_by_id(id: web::Path<i32>) -> impl Responder {
    get_data_by_id("product", id.into_inner())
        .await
        .map(|data| HttpResponse::Ok().json(data))
        .unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            HttpResponse::InternalServerError().json(json!( {
                "message": "Failed to fetch data",
                "error": e.to_string()
            }))
        })
}
