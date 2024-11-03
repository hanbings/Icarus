use crate::raft::log::LogEntry;
use crate::raft::node::Node;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct AppendRequest {
    pub leader: Node,
    pub entries: Vec<LogEntry>,
    pub term: u64,
    pub index: u64,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct AppendResponse {
    pub data: Option<String>,
    pub index: u64,
    pub success: bool,
}
