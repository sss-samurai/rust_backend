// src/routes/configure_routes.rs
use crate::controllers::create_product::create_product;
use crate::controllers::first_function::first_function;
use crate::controllers::get_product_data_by_id::get_product_data_by_id;
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/products", web::get().to(first_function)); // Use the function here
    cfg.route("/product/{id}", web::get().to(get_product_data_by_id));
    cfg.route("/product", web::post().to(create_product));
}
