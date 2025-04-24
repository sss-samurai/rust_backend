use crate::controllers::create_and_edit_product::create_and_edit_product;
use crate::controllers::delete_product::delete_product; // Corrected import
use crate::controllers::first_function::first_function;
use crate::controllers::get_product_data_by_id::get_product_data_by_id;
use crate::controllers::send_otp::send_otp;
use crate::controllers::signup_user::signup_user;
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/products", web::get().to(first_function));
    cfg.route("/product/{id}", web::get().to(get_product_data_by_id));
    cfg.route("/product", web::post().to(create_and_edit_product));
    cfg.route("/login", web::post().to(create_and_edit_product));
    cfg.route("/signup", web::post().to(signup_user));
    cfg.route("/send_otp", web::post().to(send_otp));
    cfg.route("/product/{id}", web::delete().to(delete_product));
}
