use crate::raft::config::IrisRaftConfig;
use crate::raft::log::LogEntry;
use crate::raft::node::IrisRaftNode;
use crate::raft::state::IrisRaftNodeType::Candidate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub enum IrisRaftNodeType {
    Leader,
    Follower,
    Candidate,
}

#[derive(Clone)]
pub struct IrisRaftNodeState {
    pub node: IrisRaftNode,
    pub nodes: Vec<IrisRaftNode>,
    pub raft_node_type: IrisRaftNodeType,
    pub term: usize,
    pub data: HashMap<String, String>,
    pub log: Vec<LogEntry>,
    pub config: IrisRaftConfig,
}

impl IrisRaftNodeState {
    pub fn new(config: IrisRaftConfig) -> Self {
        Self {
            node: IrisRaftNode {
                id: Uuid::new_v4(),
                created_by: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis(),
                endpoint: config.endpoint.clone(),
            },
            nodes: Vec::new(),
            raft_node_type: Candidate,
            term: 0,
            data: HashMap::new(),
            log: Vec::new(),
            config,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct IrisRaftClock {
    pub last_election_time: u128,
    pub last_heartbeat_time: u128,
    pub clock: u128,
    pub current_election_timeout_size: u128,
}

impl IrisRaftClock {
    pub fn new() -> Self {
        Self {
            // That time is updated by the append and vote interfaces.
            last_election_time: 0,
            last_heartbeat_time: 0,
            // This time is updated by the check interfaces.
            clock: 0,
            // The random timeout of vote in the raft random mechanism.
            current_election_timeout_size: 0,
        }
    }
}
