use crate::raft::append::AppendRequest;
use crate::raft::log::LogEntry;
use crate::raft::node::NodeState;
use actix_web::{get, post, web, Error, HttpResponse};
use reqwest::Client;
use std::time::Duration;
use tokio::sync::Mutex;

#[get("/")]
async fn get_state(app: web::Data<Mutex<NodeState>>) -> Result<HttpResponse, Error> {
    let node_state = app.lock().await;

    Ok(HttpResponse::Ok().json(NodeState {
        node: node_state.node.clone(),
        nodes: node_state.nodes.clone(),
        node_type: node_state.node_type.clone(),
        leader: node_state.leader.clone(),
        term: node_state.term,
        index: node_state.index,
        data: node_state.data.clone(),
    }))
}

#[get("/data")]
async fn get_data(node_state: web::Data<Mutex<NodeState>>) -> Result<HttpResponse, Error> {
    let node_state = node_state.lock().await;

    Ok(HttpResponse::Ok().json(node_state.data.clone()))
}

#[post("/data/push")]
async fn push_data(
    node_state: web::Data<Mutex<NodeState>>,
    body: web::Json<Vec<LogEntry>>,
) -> Result<HttpResponse, Error> {
    let node_state = node_state.lock().await;

    let endpoint = node_state.leader.clone();
    if endpoint.is_none() {
        return Ok(HttpResponse::Ok().json({}));
    }
    let endpoint = endpoint.unwrap().endpoint.clone();
    let leader = node_state.node.clone();
    let term = node_state.term;
    let index = node_state.index;
    let req = async move {
        let client = Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();

        client
            .post(format!("{}/append", endpoint))
            .json(&AppendRequest {
                leader,
                term,
                index,
                entries: body.clone(),
            })
            .send()
            .await
            .unwrap();
    };

    tokio::spawn(req);

    Ok(HttpResponse::Ok().json({}))
}
