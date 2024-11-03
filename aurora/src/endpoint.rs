use actix_web::{delete, get, post, web, Error, HttpResponse};
use iris_irides::raft::client::Client;
use iris_irides::raft::node::NodeState;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::Mutex;

#[get("/config")]
async fn get_config(
    node_state: web::Data<Mutex<NodeState>>,
    _client: web::Data<Mutex<Client>>,
) -> Result<HttpResponse, Error> {
    let node_state = node_state.lock().await;
    let leader = node_state.leader.clone();
    if leader.is_none() {
        return Ok(HttpResponse::Ok().json({}));
    }

    Ok(HttpResponse::Ok().json(node_state.data.clone()))
}

#[derive(Deserialize, Serialize)]
struct CreateConfigRequest {
    key: String,
    value: String,
}

#[post("/config")]
async fn post_config(
    node_state: web::Data<Mutex<NodeState>>,
    client: web::Data<Mutex<Client>>,
    body: web::Json<CreateConfigRequest>,
) -> Result<HttpResponse, Error> {
    let node_state = node_state.lock().await;
    let leader = node_state.leader.clone();
    if leader.is_none() {
        return Ok(HttpResponse::Ok().json({}));
    }

    let client = client.lock().await;
    client
        .save(
            leader.unwrap().endpoint.clone(),
            body.key.clone(),
            body.value.clone(),
        )
        .await
        .unwrap();

    Ok(HttpResponse::Ok().json(body))
}

#[get("/config/{key}")]
async fn get_config_key(
    node_state: web::Data<Mutex<NodeState>>,
    _client: web::Data<Mutex<Client>>,
    key: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let node_state = node_state.lock().await;
    let leader = node_state.leader.clone();
    if leader.is_none() {
        return Ok(HttpResponse::Ok().json({}));
    }

    let value = node_state.data.get(&key.into_inner());
    if value.is_none() {
        return Ok(HttpResponse::Ok().json({}));
    }

    Ok(HttpResponse::Ok().json(json!({ "value": value.unwrap() })))
}

#[post("/config/{key}")]
async fn post_config_key(
    node_state: web::Data<Mutex<NodeState>>,
    client: web::Data<Mutex<Client>>,
    key: web::Path<String>,
    body: web::Json<CreateConfigRequest>,
) -> Result<HttpResponse, Error> {
    let node_state = node_state.lock().await;
    let leader = node_state.leader.clone();
    if leader.is_none() {
        return Ok(HttpResponse::Ok().json({}));
    }

    let client = client.lock().await;
    client
        .update(
            leader.unwrap().endpoint.clone(),
            key.into_inner(),
            body.value.clone(),
        )
        .await
        .unwrap();

    Ok(HttpResponse::Ok().json({}))
}

#[delete("/config/{key}")]
async fn delete_config_key(
    node_state: web::Data<Mutex<NodeState>>,
    client: web::Data<Mutex<Client>>,
    key: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let node_state = node_state.lock().await;
    let leader = node_state.leader.clone();
    if leader.is_none() {
        return Ok(HttpResponse::Ok().json({}));
    }

    let client = client.lock().await;
    client
        .delete(leader.unwrap().endpoint.clone(), key.into_inner())
        .await
        .unwrap();

    Ok(HttpResponse::Ok().json({}))
}
