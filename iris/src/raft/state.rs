use crate::raft::log::LogEntry;
use crate::raft::node::IrisRaftNode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use crate::raft::config::IrisRaftConfig;
use crate::raft::state::IrisRaftNodeType::Candidate;

#[derive(Clone, Serialize, Deserialize)]
pub enum IrisRaftNodeType {
    Leader,
    Follower,
    Candidate,
}

#[derive(Clone)]
pub struct IrisRaftNodeState {
    node: IrisRaftNode,
    nodes: Vec<IrisRaftNode>,
    raft_node_type: IrisRaftNodeType,
    term: usize,
    data: HashMap<String, String>,
    log: Vec<LogEntry>,
    config: IrisRaftConfig,
}

impl IrisRaftNodeState {
    pub fn new(config: IrisRaftConfig) -> Self{
        Self {
            node: IrisRaftNode {
                id: Uuid::new_v4(),
                created_by: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis(),
                endpoint: config.endpoint.clone()
            },
            nodes: Vec::new(),
            raft_node_type: Candidate,
            term: 0,
            data: HashMap::new(),
            log: Vec::new(),
            config
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct IrisRaftClock {
    last_heartbeat: usize,
    last_election: usize,
    election_clock: usize,
    heartbeat_clock: usize,
    current_election_timeout_size: usize,
}

impl IrisRaftClock {
    pub fn new() -> Self {
        Self {
            last_heartbeat: 0,
            last_election: 0,
            election_clock: 0,
            heartbeat_clock: 0,
            current_election_timeout_size: 0,
        }
    }
}