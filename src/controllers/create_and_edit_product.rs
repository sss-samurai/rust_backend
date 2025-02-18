use crate::utils::create_data::create_data;
use crate::utils::edit_data::edit_data;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize)]
pub struct Product {
    #[allow(dead_code)] // Add this line
    drink_type: String,

    #[allow(dead_code)] // Add this line
    product_name: String,

    id: Option<i64>,
}

pub async fn create_and_edit_product(json_data: web::Json<Value>) -> impl Responder {
    let json_value = json_data.into_inner();

    let product: Result<Product, _> = serde_json::from_value(json_value.clone());

    match product {
        Ok(product) => {
            if let Some(id) = product.id {
                match edit_data("productdata", &json_value, id).await {
                    Ok(updated_product) => HttpResponse::Ok().json(json!({
                        "message": "Product updated successfully",
                        "data": updated_product
                    })),
                    Err(e) => HttpResponse::InternalServerError().json(json!({
                        "message": "Failed to update product",
                        "error": e.to_string()
                    })),
                }
            } else {
                match create_data("productdata", json_value).await {
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
        }
        Err(e) => HttpResponse::BadRequest().json(json!( {
            "error": format!("Invalid product data provided. Error: {}", e)
        })),
    }
}
