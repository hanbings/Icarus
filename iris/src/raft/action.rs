use crate::raft::log::LogEntry;
use crate::raft::node::IrisRaftNode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct AppendEntries {
    pub term: usize,
    pub leader_id: Uuid,
    pub prev_log_index: usize,
    pub prev_log_term: usize,
    pub entries: Vec<LogEntry>,
    pub leader_commit_index: usize,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AppendEntriesResponse {
    pub term: usize,
    pub success: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RequestVote {
    pub node: IrisRaftNode,
    pub term: usize,
    pub candidate_id: Uuid,
    pub last_log_index: usize,
    pub last_log_term: usize,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RequestVoteResponse {
    pub term: usize,
    pub vote_granted: bool,
}
