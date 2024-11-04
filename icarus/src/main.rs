mod config;
mod endpoint;

use actix_web::{App, HttpServer};
use figment::providers::{Format, Json, Toml};
use figment::Figment;
use log::{error, info};
use std::env;
use std::env::set_var;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    set_var("RUST_LOG", "info");
    env_logger::init();

    let env_config = env::var("AURORA_CONFIG");
    let env_config = env_config.unwrap_or_else(|_| "{}".to_string());

    info!("Extracting config...");
    let config: config::Config = Figment::new()
        .merge(Toml::file("icarus.toml"))
        .merge(Json::string(env_config.as_str()))
        .extract()
        .unwrap_or_else(|_| {
            error!(
                "Unable to extract config from {}.\
                Check aurora.toml or set AURORA_CONFIG environment variable.",
                env_config
            );
            std::process::exit(1);
        });

    info!("Application Running...");
    HttpServer::new(move || App::new())
        .bind((config.ip, config.port))?
        .run()
        .await
}
