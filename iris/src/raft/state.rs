use crate::raft::config::IrisRaftConfig;
use crate::raft::log::LogEntry;
use crate::raft::node::IrisRaftNode;
use crate::raft::state::IrisRaftNodeType::Candidate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Serialize, Deserialize)]
pub enum IrisRaftNodeType {
    Leader,
    Follower,
    Candidate,
}

#[derive(Clone)]
pub struct IrisRaftNodeState {
    // node
    pub node: IrisRaftNode,
    pub nodes: Vec<IrisRaftNode>,
    pub leader_id: Option<String>,
    pub raft_node_type: IrisRaftNodeType,

    // state
    pub term: usize,
    pub voted_for: Option<String>,
    pub log: Vec<LogEntry>,
    pub commit_index: usize,
    pub last_applied_index: usize,

    // data
    pub data: HashMap<String, String>,

    // config
    pub config: IrisRaftConfig,
}

impl IrisRaftNodeState {
    pub fn new(config: IrisRaftConfig) -> Self {
        Self {
            node: IrisRaftNode {
                id: config.id.clone(),
                created_by: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis(),
                endpoint: config.endpoint.clone(),
            },
            nodes: Vec::new(),
            raft_node_type: Candidate,
            leader_id: None,
            term: 0,
            voted_for: None,
            log: Vec::new(),
            commit_index: 0,
            last_applied_index: 0,
            data: HashMap::new(),
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
