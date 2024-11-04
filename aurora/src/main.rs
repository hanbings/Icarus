use crate::endpoint::{
    delete_config_key, get_config, get_config_key, post_config, post_config_key,
};
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use figment::providers::{Format, Json, Toml};
use figment::Figment;
use iris_irides::raft::client;
use iris_irides::raft::node::{Node, NodeClockState, NodeState, NodeType};
use iris_irides::security::secret::secret_middleware;
use log::info;
use std::collections::HashMap;
use std::env;
use std::env::set_var;
use std::time::SystemTime;
use tokio::sync::Mutex;

mod config;
mod endpoint;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    set_var("RUST_LOG", "info");
    env_logger::init();

    let env_config = env::var("AURORA_CONFIG");
    let env_config = env_config.unwrap_or_else(|_| "{}".to_string());

    info!("Extracting config...");
    let config: config::Config = Figment::new()
        .merge(Toml::file("aurora.toml"))
        .merge(Json::string(env_config.as_str()))
        .extract()
        .unwrap();

    info!("Initializing client...");
    tokio::spawn(client::async_clock(
        config.endpoint.clone(),
        config.secret.clone(),
    ));

    let node = Node {
        endpoint: config.endpoint.clone(),
    };
    let nodes = config
        .nodes
        .iter()
        .map(|node| Node {
            endpoint: node.clone(),
        })
        .collect();
    info!("Setting up server...");
    let node_state = Data::new(Mutex::new(NodeState {
        node,
        nodes,
        node_type: NodeType::Follower,
        leader: None,
        term: 0,
        index: 0,
        log: vec![],
        data: HashMap::new(),
        secret: config.secret.clone(),
    }));
    let node_clock = Data::new(Mutex::new(NodeClockState {
        clock: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis(),
        heartbeat: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis(),
        election: 0,
    }));
    let client = Data::new(Mutex::new(client::Client {}));

    info!("Application Running...");
    HttpServer::new(move || {
        let auth = HttpAuthentication::with_fn(secret_middleware);
        App::new()
            .app_data(node_state.clone())
            .app_data(node_clock.clone())
            .app_data(client.clone())
            .wrap(auth)
            .service(iris_irides::raft::endpoint_append::append)
            .service(iris_irides::raft::endpoint_vote::vote)
            .service(iris_irides::raft::endpoint_check::check)
            .service(iris_irides::raft::endpoint_metadata::get_state)
            .service(iris_irides::raft::endpoint_metadata::get_data)
            .service(iris_irides::raft::endpoint_metadata::post_data)
            .service(get_config)
            .service(get_config_key)
            .service(post_config)
            .service(post_config_key)
            .service(delete_config_key)
    })
    .bind((config.ip, config.port))?
    .run()
    .await
}
