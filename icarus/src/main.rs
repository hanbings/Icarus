mod config;
mod endpoint;
mod message;
mod security;

use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use endpoint::config_center::{
    config_create_config, config_delete_config_key, config_get_config, config_update_config_key,
};
use endpoint::login::login;
use endpoint::message_queue::message_get_queue;
use endpoint::service_explore::explore_service_get_services;
use figment::providers::{Format, Json, Toml};
use figment::Figment;
use log::{error, info};
use security::secret::secret_middleware;
use std::env;
use std::env::set_var;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    set_var("RUST_LOG", "info");
    env_logger::init();

    let env_config = env::var("ICARUS_CONFIG");
    let env_config = env_config.unwrap_or_else(|_| "{}".to_string());

    info!("Extracting config...");
    let config: config::Config = Figment::new()
        .merge(Toml::file("icarus.toml"))
        .merge(Json::string(env_config.as_str()))
        .extract()
        .unwrap_or_else(|_| {
            error!(
                "Unable to extract config from {}.\
                Check icarus.toml or set ICARUS_CONFIG environment variable.",
                env_config
            );
            std::process::exit(1);
        });

    info!("Initializing client...");
    let config_state = Data::new(config.clone());

    info!("Application Running...");
    HttpServer::new(move || {
        let auth = HttpAuthentication::with_fn(secret_middleware);
        App::new()
            .app_data(config_state.clone())
            .wrap(Cors::permissive())
            .wrap(auth)
            .service(config_get_config)
            .service(config_create_config)
            .service(config_update_config_key)
            .service(config_delete_config_key)
            .service(explore_service_get_services)
            .service(message_get_queue)
            .service(login)
    })
    .bind((config.ip, config.port))?
    .run()
    .await
}
