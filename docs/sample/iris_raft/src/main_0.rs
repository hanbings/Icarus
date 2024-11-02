use crate::state::{AppState, ClockState, Node, NodeType};
use actix_web::{App, HttpServer};
use std::env::set_var;
use std::time::SystemTime;
use tokio::sync::Mutex;

mod client;
mod server;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    set_var("RUST_LOG", "info");
    env_logger::init();

    let port = 10001;

    tokio::spawn(client::async_clock(
        format!("http://127.0.0.1:{}/check", port).to_string(),
    ));

    let app_state = actix_web::web::Data::new(Mutex::new(AppState {
        node: Node {
            endpoint: format!("http://127.0.0.1:{}", port).to_string(),
        },
        nodes: vec![
            Node {
                endpoint: "http://127.0.0.1:10000".to_string(),
            },
            Node {
                endpoint: "http://127.0.0.1:10001".to_string(),
            },
            Node {
                endpoint: "http://127.0.0.1:10002".to_string(),
            },
        ],
        node_type: NodeType::Follower,
        leader: None,
        term: 0,
        index: 0,
        data: Vec::new(),
    }));

    let clock_state = actix_web::web::Data::new(Mutex::new(ClockState {
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

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .app_data(clock_state.clone())
            .service(server::check)
            .service(server::vote)
            .service(server::append)
            .service(server::get_state)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
