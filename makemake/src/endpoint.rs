use crate::message::Message;
use crate::raft::client::Client;
use crate::raft::node::NodeState;
use actix_web::{delete, get, post, web, Error, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct DataRequest {
    channel: String,
    value: String,
}

#[get("/message")]
pub async fn get_queue(
    node_state: web::Data<Mutex<NodeState>>,
    auth: BearerAuth,
) -> Result<HttpResponse, Error> {
    let node_state = node_state.lock().await;

    if node_state.secret.is_some() && *auth.token() != node_state.secret.clone().unwrap() {
        return Ok(HttpResponse::Unauthorized().json(Message::unauthorized()));
    }

    Ok(HttpResponse::Ok().json(node_state.data.clone()))
}

#[post("/message/{channel}/push")]
pub async fn push_queue(
    node_state: web::Data<Mutex<NodeState>>,
    client: web::Data<Mutex<Client>>,
    body: web::Json<DataRequest>,
    channel: web::Path<String>,
    auth: BearerAuth,
) -> Result<HttpResponse, Error> {
    let node_state = node_state.lock().await;

    if node_state.secret.is_some() && *auth.token() != node_state.secret.clone().unwrap() {
        return Ok(HttpResponse::Unauthorized().json(Message::unauthorized()));
    }

    let leader = node_state.leader.clone();
    if leader.is_none() {
        return Ok(HttpResponse::Ok().json(Message::fail()));
    }

    let client = client.lock().await;
    client
        .push(
            leader.unwrap().endpoint.clone(),
            channel.into_inner(),
            body.value.clone(),
            node_state.secret.clone(),
        )
        .await
        .unwrap();

    Ok(HttpResponse::Ok().json(Message::success()))
}

#[get("/message/{channel}/pop")]
pub async fn pop_queue(
    node_state: web::Data<Mutex<NodeState>>,
    client: web::Data<Mutex<Client>>,
    channel: web::Path<String>,
    auth: BearerAuth,
) -> Result<HttpResponse, Error> {
    let node_state = node_state.lock().await;

    if node_state.secret.is_some() && *auth.token() != node_state.secret.clone().unwrap() {
        return Ok(HttpResponse::Unauthorized().json(Message::unauthorized()));
    }

    let leader = node_state.leader.clone();
    if leader.is_none() {
        return Ok(HttpResponse::Ok().json(Message::fail()));
    }

    let client = client.lock().await;
    let res = client
        .pop(
            leader.unwrap().endpoint.clone(),
            channel.into_inner(),
            node_state.secret.clone(),
        )
        .await;

    match res {
        Ok(pop_data) => Ok(HttpResponse::Ok().json(pop_data)),
        Err(_) => Ok(HttpResponse::Ok().json(Message::fail())),
    }
}

#[post("/message/{channel}")]
pub async fn update_queue(
    node_state: web::Data<Mutex<NodeState>>,
    client: web::Data<Mutex<Client>>,
    channel: web::Path<String>,
    body: web::Json<DataRequest>,
    auth: BearerAuth,
) -> Result<HttpResponse, Error> {
    let node_state = node_state.lock().await;

    if node_state.secret.is_some() && *auth.token() != node_state.secret.clone().unwrap() {
        return Ok(HttpResponse::Unauthorized().json(Message::unauthorized()));
    }

    let leader = node_state.leader.clone();
    if leader.is_none() {
        return Ok(HttpResponse::Ok().json(Message::fail()));
    }

    let client = client.lock().await;
    client
        .update(
            leader.unwrap().endpoint.clone(),
            channel.into_inner(),
            body.value.clone(),
            node_state.secret.clone(),
        )
        .await
        .unwrap();

    Ok(HttpResponse::Ok().json(Message::success()))
}

#[delete("/message/{channel}")]
pub async fn delete_queue(
    node_state: web::Data<Mutex<NodeState>>,
    client: web::Data<Mutex<Client>>,
    channel: web::Path<String>,
    auth: BearerAuth,
) -> Result<HttpResponse, Error> {
    let node_state = node_state.lock().await;

    if node_state.secret.is_some() && *auth.token() != node_state.secret.clone().unwrap() {
        return Ok(HttpResponse::Unauthorized().json(Message::unauthorized()));
    }

    let leader = node_state.leader.clone();
    if leader.is_none() {
        return Ok(HttpResponse::Ok().json(Message::fail()));
    }

    let client = client.lock().await;
    client
        .delete(
            leader.unwrap().endpoint.clone(),
            channel.into_inner(),
            node_state.secret.clone(),
        )
        .await
        .unwrap();

    Ok(HttpResponse::Ok().json(Message::success()))
}
