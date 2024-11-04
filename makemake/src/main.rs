use crate::endpoint::{delete_queue, get_queue, pop_queue, push_queue, update_queue};
use crate::raft::client;
use crate::raft::node::{Node, NodeClockState, NodeState, NodeType};
use crate::security::secret::secret_middleware;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use figment::providers::{Format, Json, Toml};
use figment::Figment;
use log::{error, info};
use std::collections::HashMap;
use std::env;
use std::env::set_var;
use std::time::SystemTime;
use tokio::sync::Mutex;

mod config;
mod endpoint;
mod message;
mod raft;
mod security;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    set_var("RUST_LOG", "info");
    env_logger::init();

    let env_config = env::var("MAKEMAKE_CONFIG");
    let env_config = env_config.unwrap_or_else(|_| "{}".to_string());

    info!("Extracting config...");
    let config: config::Config = Figment::new()
        .merge(Toml::file("makemake.toml"))
        .merge(Json::string(env_config.as_str()))
        .extract()
        .unwrap_or_else(|_| {
            error!(
                "Unable to extract config from {}.\
                Check makemake.toml or set MAKEMAKE_CONFIG environment variable.",
                env_config
            );
            std::process::exit(1);
        });

    info!("Initializing client...");
    tokio::spawn(client::async_clock(
        config.endpoint.clone(),
        config.secret.clone(),
    ));

    info!("Setting up server...");
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
            .service(raft::endpoint_append::append)
            .service(raft::endpoint_vote::vote)
            .service(raft::endpoint_check::check)
            .service(raft::endpoint_metadata::get_state)
            .service(raft::endpoint_metadata::get_data)
            .service(raft::endpoint_metadata::post_data)
            .service(get_queue)
            .service(push_queue)
            .service(pop_queue)
            .service(update_queue)
            .service(delete_queue)
    })
    .bind((config.ip, config.port))?
    .run()
    .await
}
