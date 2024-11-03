use actix_web::web::Data;
use actix_web::{App, HttpServer};
use figment::providers::{Format, Toml};
use figment::Figment;
use iris_irides::raft::node::{Node, NodeClockState, NodeState, NodeType};
use log::info;
use std::collections::HashMap;
use std::env::set_var;
use std::time::SystemTime;
use tokio::sync::Mutex;
use iris_irides::raft::client;
use iris_irides::raft::endpoint_append::append;
use iris_irides::raft::endpoint_check::check;
use iris_irides::raft::endpoint_metadata::{get_data, get_state, push_data};
use iris_irides::raft::endpoint_vote::vote;

mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    set_var("RUST_LOG", "info");
    env_logger::init();

    info!("Extracting config...");
    let config: config::Config = Figment::new()
        .merge(Toml::file("aurora_1.toml"))
        .extract()
        .unwrap();

    info!("Initializing client...");
    tokio::spawn(client::async_clock(
        format!("http://127.0.0.1:{}/check", config.port).to_string(),
    ));

    let node = Node { endpoint: config.endpoint.clone() };
    let nodes = config.nodes.iter().map(|node| Node { endpoint: node.clone() }).collect();
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

    info!("Application Running...");
    HttpServer::new(move ||
        App::new()
            .app_data(node_state.clone())
            .app_data(node_clock.clone())
            .service(append)
            .service(vote)
            .service(check)
            .service(get_state)
            .service(get_data)
            .service(push_data)
    )
        .bind((config.ip, config.port))?
        .run()
        .await
}
