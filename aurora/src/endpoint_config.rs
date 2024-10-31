use actix_web::{web, web::Data, Responder};
use iris_irides::message::Message;
use iris_irides::raft::client::IrisRaftClient;
use iris_irides::raft::state::IrisRaftNodeState;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct ConfigData {
    key: String,
    value: Option<String>,
}

pub async fn get_config(
    node_state: Data<Mutex<IrisRaftNodeState>>,
) -> actix_web::Result<impl Responder> {
    let node_state = node_state.lock().unwrap();

    Ok(web::Json(serde_json::json!(node_state.data.clone())))
}

#[derive(Serialize, Deserialize)]
struct ConfigDataResponse {
    key: String,
    value: String,
}

pub async fn get_config_by_id(
    node_state: Data<Mutex<IrisRaftNodeState>>,
    path: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let node_state = node_state.lock().unwrap();
    let key = path.into_inner();

    if node_state.data.get(&key).is_none() {
        return Ok(web::Json(serde_json::json!(Message::fail())));
    }

    let value = node_state.data.get(&key).unwrap();
    let config = ConfigDataResponse {
        key: key.clone(),
        value: value.clone(),
    };

    Ok(web::Json(serde_json::json!(config)))
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub key: String,
    pub value: Option<String>,
}

pub async fn post_config(
    client: Data<Mutex<IrisRaftClient>>,
    body: web::Json<Config>,
) -> actix_web::Result<impl Responder> {
    let client = client.lock().unwrap();

    if body.value.is_none() {
        return Ok(web::Json({}));
    }

    client
        .save(body.key.clone(), body.value.clone().unwrap())
        .await;

    Ok(web::Json({}))
}
pub async fn update_config(
    client: Data<Mutex<IrisRaftClient>>,
    path: web::Path<String>,
    body: web::Json<Config>,
) -> actix_web::Result<impl Responder> {
    let client = client.lock().unwrap();

    if body.value.is_none() || body.key != path.into_inner() {
        return Ok(web::Json(serde_json::json!(Message::fail())));
    }

    client
        .update(body.key.clone(), body.value.clone().unwrap())
        .await;

    Ok(web::Json(serde_json::json!(Message::success())))
}
pub async fn delete_config(
    client: Data<Mutex<IrisRaftClient>>,
    path: web::Path<String>,
    body: web::Json<Config>,
) -> actix_web::Result<impl Responder> {
    let client = client.lock().unwrap();

    if body.key != path.into_inner() {
        return Ok(web::Json(serde_json::json!(Message::fail())));
    }

    client.delete(body.key.clone()).await;

    Ok(web::Json(serde_json::json!(Message::success())))
}
