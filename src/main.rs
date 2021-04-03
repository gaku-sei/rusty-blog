#[macro_use]
extern crate diesel;
extern crate juniper;

mod app;
mod db;
mod graphql;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use graphql::create_schema;
use itconfig::config;

config! {
    DEBUG: bool => true,
    DATABASE {
        URL: String
    },
    PORT: u32 => 8000
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect("dotenv setup to be successful");

    HttpServer::new(move || {
        App::new()
            .data(create_schema())
            .data(db::connect())
            .configure(app::register)
    })
    .bind(format!("0.0.0.0:{}", config::PORT()))?
    .run()
    .await
}
