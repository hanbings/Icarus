#![allow(dead_code)]
use crate::gossip::node::GossipNode;
use crate::gossip::state::ClusterState;
use actix_web::web::Data;
use actix_web::{get, web, Responder, Result};
use std::sync::Mutex;

#[get("/node")]
pub async fn get_node(data: Data<Mutex<ClusterState>>) -> Result<impl Responder> {
    let node = &data.lock().unwrap().node;

    Ok(web::Json(node.clone()))
}

#[get("/nodes")]
pub async fn get_nodes(data: Data<Mutex<ClusterState>>) -> Result<impl Responder> {
    let nodes = &mut data.lock().unwrap().nodes;
    let node = GossipNode::default();
    nodes.insert(node.id, node);

    Ok(web::Json(nodes.clone()))
}
