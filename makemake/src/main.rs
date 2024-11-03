use crate::endpoint::{delete_queue, get_queue, pop_queue, push_queue, update_queue};
use crate::raft::client;
use crate::security::secret::secret_middleware;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use log::info;
use tokio::sync::Mutex;

mod endpoint;
mod message;
mod raft;
mod security;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    info!("Initializing client...");
    tokio::spawn(client::async_clock(
        "http://127.0.0.1:8080".to_string(),
        None,
    ));

    info!("Setting up server...");
    let client = Data::new(Mutex::new(client::Client {}));

    info!("Application Running...");
    HttpServer::new(move || {
        let auth = HttpAuthentication::with_fn(secret_middleware);
        App::new()
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
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
