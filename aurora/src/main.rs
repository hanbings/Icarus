use crate::endpoint_config::{delete_config, get_config, post_config, update_config};
use actix_web::web::{to, Data};
use actix_web::{web, App, HttpServer};
use endpoint_config::get_config_by_id;
use endpoint_metadata::status;
use figment::providers::{Format, Toml};
use figment::Figment;
use iris_irides::raft::client::IrisRaftClient;
use iris_irides::raft::config::IrisRaftConfig;
use iris_irides::raft::state::{IrisRaftClock, IrisRaftNodeState};
use log::info;
use std::collections::HashMap;
use std::env::set_var;
use std::sync::Mutex;
use std::time::Duration;
use tokio::time;

mod config;
mod endpoint_config;
mod endpoint_metadata;

pub struct ClientState {
    config: HashMap<String, String>,
    client: IrisRaftClient,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    set_var("RUST_LOG", "info");
    env_logger::init();

    info!("Extracting config...");
    let config: config::Config = Figment::new()
        .merge(Toml::file("aurora.toml"))
        .extract()
        .unwrap();

    let endpoint = config.endpoint.clone();

    info!("Generating clock task...");
    tokio::spawn(IrisRaftClient::async_clock(endpoint));

    info!("Initializing state...");
    let clock = Data::new(Mutex::new(IrisRaftClock::new()));
    let client = Data::new(Mutex::new(IrisRaftClient::new()));
    let node_state = Data::new(Mutex::new(IrisRaftNodeState::new(
        IrisRaftConfig::no_log_compaction(
            config.node,
            config.secret,
            config.endpoint,
            200,
            (300, 800),
        ),
    )));

    info!("Application Running...");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&clock))
            .app_data(Data::clone(&client))
            .app_data(Data::clone(&node_state))
            // metadata
            .route(
                "/cluster/node",
                web::get().to(iris_irides::raft::endpoint_metadata::get_node),
            )
            .route(
                "/cluster/nodes",
                web::get().to(iris_irides::raft::endpoint_metadata::get_nodes),
            )
            .route(
                "/cluster/status",
                web::get().to(iris_irides::raft::endpoint_metadata::get_status),
            )
            // check
            .route(
                "/cluster/check",
                web::post().to(iris_irides::raft::endpoint_check::post_check),
            )
            // action
            .route(
                "/cluster/append",
                web::post().to(iris_irides::raft::endpoint_action::post_append),
            )
            .route(
                "/cluster/commit",
                web::post().to(iris_irides::raft::endpoint_action::post_commit),
            )
            .route(
                "/cluster/vote",
                web::post().to(iris_irides::raft::endpoint_action::post_vote),
            )
            // config
            .route("/config", web::get().to(get_config))
            .route("/config", web::post().to(post_config))
            .route("/config/{id}", web::post().to(update_config))
            .route("/config/{id}", web::get().to(get_config_by_id))
            .route("/config/{id}", web::delete().to(delete_config))
            // status
            .route("/status", web::get().to(status))
    })
    .bind((config.ip, config.port))?
    .run()
    .await
}
