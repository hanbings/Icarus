use crate::gossip::state::ClusterState;
use actix_web::{web, Responder, Result};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
pub struct DataKey {
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub message: String,
    pub code: i32,
}

pub async fn get_data(
    cluster_state: web::Data<Mutex<&mut ClusterState>>,
) -> Result<impl Responder> {
    let node = cluster_state.lock().unwrap();
    let data = &node.data;

    Ok(web::Json(data.clone()))
}

pub async fn get_data_with_key(
    path: web::Query<DataKey>,
    cluster_state: web::Data<Mutex<&mut ClusterState>>,
) -> Result<impl Responder> {
    let node = cluster_state.lock().unwrap();
    let _saving = &node.data.get(&path.key).unwrap().clone();

    Ok(web::Json(Message {
        message: "success".to_string(),
        code: 200,
    }))
}

pub async fn post_create_or_update_data_with_key(
    path: web::Query<Data>,
    cluster_state: web::Data<Mutex<&mut ClusterState>>,
) -> Result<impl Responder> {
    let mut node = cluster_state.lock().unwrap();
    let path = path.0;
    node.data.insert(path.key, path.value).unwrap();

    Ok(web::Json(Message {
        message: "success".to_string(),
        code: 200,
    }))
}

pub async fn delete_data(
    path: web::Query<DataKey>,
    cluster_state: web::Data<Mutex<&mut ClusterState>>,
) -> Result<impl Responder> {
    let mut node = cluster_state.lock().unwrap();
    node.data.remove(&path.key);

    Ok(web::Json(Message {
        message: "success".to_string(),
        code: 200,
    }))
}
