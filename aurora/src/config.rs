use iris_irides::raft::node::IrisRaftNode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub ip: String,
    pub port: u16,
    pub id: String,
    pub endpoint: String,
    pub secret: String,
    pub nodes: Vec<IrisRaftNode>,
}
