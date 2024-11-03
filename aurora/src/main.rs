use actix_web::{App, HttpServer};
use figment::providers::{Format, Toml};
use figment::Figment;
use log::info;
use std::env::set_var;

mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    set_var("RUST_LOG", "info");
    env_logger::init();

    info!("Extracting config...");
    let config: config::Config = Figment::new()
        .merge(Toml::file("aurora.toml"))
        .extract()
        .unwrap();

    info!("Application Running...");
    HttpServer::new(move || App::new())
        .bind((config.ip, config.port))?
        .run()
        .await
}
