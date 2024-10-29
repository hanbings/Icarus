use actix_web::web::Data;
use actix_web::Responder;
use iris_irides::raft::node::IrisRaftNode;
use iris_irides::raft::state::{IrisRaftNodeState, IrisRaftNodeType};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct Status {
    pub node: IrisRaftNode,
    pub nodes: Vec<IrisRaftNode>,
    pub raft_node_type: IrisRaftNodeType,
    pub term: usize,
}

pub async fn status(
    node_state: Data<Mutex<IrisRaftNodeState>>,
) -> actix_web::Result<impl Responder> {
    let node_state = node_state.lock().unwrap();

    let status = Status {
        node: node_state.node.clone(),
        nodes: node_state.nodes.clone(),
        raft_node_type: node_state.raft_node_type.clone(),
        term: node_state.term,
    };

    Ok(actix_web::web::Json(status))
}
