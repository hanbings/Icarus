use iris_irides::raft::node::Node;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub ip: String,
    pub port: u16,
    pub node: Node,
    pub nodes: Vec<Node>,
}
