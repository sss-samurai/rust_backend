use crate::utils::delete_data::delete_data;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

pub async fn delete_product(id: web::Path<i64>) -> impl Responder {
    match delete_data("productdata", *id).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "message": "Product deleted successfully",
            "product_id": id.into_inner()
        })),
        Err(_) => HttpResponse::NotFound().json(json!({
            "error": "Product not found or could not be deleted",
            "product_id": id.into_inner()
        })),
    }
}
