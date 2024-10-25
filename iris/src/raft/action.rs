use crate::raft::log::LogEntry;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct AppendEntries {
    term: usize,
    leader_id: usize,
    prev_log_index: usize,
    prev_log_term: usize,
    entries: Vec<LogEntry>,
    leader_commit_index: usize,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AppendEntriesResponse {
    term: usize,
    success: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RequestVote {
    term: usize,
    candidate_id: usize,
    last_log_index: usize,
    last_log_term: usize,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RequestVoteResponse {
    term: usize,
    vote_granted: bool,
}