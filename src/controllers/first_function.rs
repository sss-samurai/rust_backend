use crate::utils::get_all_data::get_all_data;
use actix_web::{HttpResponse, Responder};
use serde_json::{json, Value};

pub async fn first_function() -> impl Responder {
    let table_name = "coffe";
    match get_all_data(table_name).await {
        Ok(data) => {
            if let Value::Array(ref arr) = data {
                if arr.is_empty() {
                    HttpResponse::NotFound().json(json!({
                        "message": "No data found",
                        "error": "No data found in the database"
                    }))
                } else {
                    HttpResponse::Ok().json(json!({
                        "message": "Data fetched successfully",
                        "data": data
                    }))
                }
            } else {
                HttpResponse::InternalServerError().json(json!({
                    "message": "Unexpected data format",
                    "error": "Something went wrong while fetching data"
                }))
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "message": "Failed to fetch data",
                "error": e.to_string()
            }))
        }
    }
}
