use crate::gossip::node::GossipNode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct ClusterState {
    pub node: GossipNode,
    pub neighbors: Vec<GossipNode>,
    pub data: HashMap<String, String>,
    pub nodes: HashMap<Uuid, GossipNode>,
}