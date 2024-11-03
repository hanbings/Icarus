use crate::raft::log::LogEntry;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub endpoint: String,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    Follower,
    Candidate,
    Leader,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct NodeState {
    pub node: Node,
    pub nodes: Vec<Node>,
    pub node_type: NodeType,
    pub leader: Option<Node>,
    pub term: u64,
    // In order to avoid cloning data back and forth during the asynchronous execution
    // of sending append entries,
    // iris does not implement the steps to confirm the term and
    // log_index of the append entries,
    // so the last_term_index field is not needed.
    pub index: u64,
    pub data: Vec<LogEntry>,
}

impl NodeState {
    pub fn set_candidate(&mut self) {
        self.node_type = NodeType::Candidate;
        self.term += 1;
    }

    pub fn set_follower(&mut self, leader: Node, term: u64, index: u64) {
        self.node_type = NodeType::Follower;
        self.leader = Some(leader);
        self.term = term;
        self.index = index;
    }

    pub fn set_leader(&mut self) {
        self.node_type = NodeType::Leader;
        self.leader = Some(self.node.clone());
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct NodeClockState {
    pub clock: u128,
    pub heartbeat: u128,
    pub election: u128,
}

impl NodeClockState {
    pub fn update_clock(&mut self) {
        self.clock = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();
    }

    pub fn update_heartbeat(&mut self) {
        self.heartbeat = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();
    }

    pub fn update_election(&mut self) {
        self.election = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();
    }
}
