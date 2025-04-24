use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
mod controllers;
mod models;
mod routes;
mod utils;
use routes::configure_routes::configure_routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    println!("SSS is running...");
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(configure_routes)
    })
    // .workers(12)
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
