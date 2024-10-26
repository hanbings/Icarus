use crate::gossip::node::GossipNode;
use crate::gossip::state::ClusterState;
use actix_web::web::Data;
use actix_web::{post, web, Responder, Result};
use std::sync::Mutex;

pub async fn post_connect(
    node: web::Json<GossipNode>,
    cluster_state: Data<Mutex<ClusterState>>,
) -> Result<impl Responder> {
    let nodes = &mut cluster_state.lock().unwrap().nodes;
    let gossip_node = GossipNode {
        id: node.id,
        host: node.host,
        port: node.port,
        created_by: node.created_by,
    };
    let _ = &nodes.insert(gossip_node.id, gossip_node);

    Ok(web::Json(nodes.clone()))
}

pub async fn post_disconnect(
    node: web::Json<GossipNode>,
    cluster_state: Data<Mutex<ClusterState>>,
) -> Result<impl Responder> {
    let nodes = &mut cluster_state.lock().unwrap().nodes;
    let _ = &nodes.remove(&node.id);

    Ok(web::Json(nodes.clone()))
}
