use crate::message::Message;
use crate::raft::append::{AppendRequest, AppendResponse};
use crate::raft::client::PopData;
use crate::raft::log::LogEntry;
use crate::raft::node::NodeState;
use actix_web::{get, post, web, Error, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use reqwest::Client;
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

    let secret = node_state.secret.clone();
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let mut req = client
        .post(format!("{}/raft/append", endpoint.unwrap().endpoint))
        .json(&AppendRequest {
            leader: node_state.node.clone(),
            term: node_state.term,
            index: node_state.index,
            entries: body.clone(),
        });

    if secret.is_some() {
        req = req.bearer_auth(secret.clone().unwrap());
    }

    let res = req.send().await.unwrap();
    let res = res.json::<AppendResponse>().await.unwrap();

    if res.success && res.data.is_some() {
        return Ok(HttpResponse::Ok().json(PopData {
            data: res.data,
            success: res.success,
        }));
    }

    Ok(HttpResponse::Ok().json(Message::success()))
}
