#![allow(dead_code)]

use crate::raft::node::IrisRaftNode;
use crate::raft::state::{IrisRaftNodeState, IrisRaftNodeType};
use actix_web::web::Data;
use actix_web::Responder;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

pub async fn get_node(
    node_state: Data<Mutex<IrisRaftNodeState>>,
) -> actix_web::Result<impl Responder> {
    Ok(actix_web::web::Json(serde_json::json!(node_state
        .lock()
        .unwrap()
        .node
        .clone())))
}
pub async fn get_nodes(
    node_state: Data<Mutex<IrisRaftNodeState>>,
) -> actix_web::Result<impl Responder> {
    Ok(actix_web::web::Json(serde_json::json!(node_state
        .lock()
        .unwrap()
        .nodes
        .clone())))
}

#[derive(Serialize, Deserialize)]
struct Status {
    pub node: IrisRaftNode,
    pub nodes: Vec<IrisRaftNode>,
    pub raft_node_type: IrisRaftNodeType,
    pub term: usize,
}
pub async fn get_status(
    node_state: Data<Mutex<IrisRaftNodeState>>,
) -> actix_web::Result<impl Responder> {
    let node_state = node_state.lock().unwrap();

    let status = Status {
        node: node_state.node.clone(),
        nodes: node_state.nodes.clone(),
        raft_node_type: node_state.raft_node_type.clone(),
        term: node_state.term,
    };

    Ok(actix_web::web::Json(serde_json::json!(status)))
}
