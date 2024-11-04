use crate::message::Message;
use crate::raft::append::AppendRequest;
use crate::raft::client::PopData;
use crate::raft::log::LogEntry;
use crate::raft::node::NodeState;
use actix_web::{get, post, web, Error, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use log::info;
use reqwest::Client;
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::Mutex;

#[get("/raft/status")]
async fn get_state(
    app: web::Data<Mutex<NodeState>>,
    auth: BearerAuth,
) -> Result<HttpResponse, Error> {
    let node_state = app.lock().await;

    if node_state.secret.is_some() && *auth.token() != node_state.secret.clone().unwrap() {
        return Ok(HttpResponse::Unauthorized().json(Message::unauthorized()));
    }

    Ok(HttpResponse::Ok().json(NodeState {
        node: node_state.node.clone(),
        nodes: node_state.nodes.clone(),
        node_type: node_state.node_type.clone(),
        leader: node_state.leader.clone(),
        term: node_state.term,
        index: node_state.index,
        log: node_state.log.clone(),
        data: node_state.data.clone(),
        secret: None,
    }))
}

#[get("/raft/data")]
async fn get_data(
    node_state: web::Data<Mutex<NodeState>>,
    auth: BearerAuth,
) -> Result<HttpResponse, Error> {
    let node_state = node_state.lock().await;

    if node_state.secret.is_some() && *auth.token() != node_state.secret.clone().unwrap() {
        return Ok(HttpResponse::Unauthorized().json(Message::unauthorized()));
    }

    Ok(HttpResponse::Ok().json(node_state.data.clone()))
}

#[post("/raft/data")]
async fn post_data(
    node_state: web::Data<Mutex<NodeState>>,
    body: web::Json<Vec<LogEntry>>,
    auth: BearerAuth,
) -> Result<HttpResponse, Error> {
    let node_state = node_state.lock().await;

    if node_state.secret.is_some() && *auth.token() != node_state.secret.clone().unwrap() {
        return Ok(HttpResponse::Unauthorized().json(Message::unauthorized()));
    }

    let endpoint = node_state.leader.clone();
    if endpoint.is_none() {
        return Ok(HttpResponse::Ok().json(Message::fail()));
    }

    let endpoint = endpoint.unwrap().endpoint.clone();
    let leader = node_state.node.clone();
    let term = node_state.term;
    let index = node_state.index;
    let secret = node_state.secret.clone();
    let req = async move {
        let client = Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();

        let mut req = client
            .post(format!("{}/raft/append", endpoint))
            .json(&AppendRequest {
                leader,
                term,
                index,
                entries: body.clone(),
            });

        if secret.is_some() {
            req = req.bearer_auth(secret.clone().unwrap());
        }

        req.send().await.unwrap();
    };

    tokio::spawn(req);

    Ok(HttpResponse::Ok().json(Message::success()))
}

#[get("/raft/data/pop/{token}")]
async fn pop_data(
    pop_state: web::Data<Mutex<HashMap<String, String>>>,
    token: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let pop_state = pop_state.lock().await;

    info!("Pop state: {:?}", pop_state);

    let pop_data = pop_state.get(&token.into_inner());
    if pop_data.is_none() {
        return Ok(HttpResponse::Ok().json(Message::fail()));
    }

    Ok(HttpResponse::Ok().json(PopData {
        data: pop_data.unwrap().clone(),
    }))
}
