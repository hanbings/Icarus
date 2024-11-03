use crate::raft::node::Node;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct VoteRequest {
    pub candidate: Node,
    pub term: u64,
    pub index: u64,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct VoteResponse {
    pub leader: Option<Node>,
    pub granted: bool,
    pub term: u64,
    pub index: u64,
}
