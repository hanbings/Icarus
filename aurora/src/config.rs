use serde::{Deserialize, Serialize};
use iris_irides::raft::node::IrisRaftNode;

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub ip: String,
    pub port: u16,
    pub id: String,
    pub endpoint: String,
    pub secret: String,
    pub nodes: Vec<IrisRaftNode>,
}
