#![allow(dead_code)]
use crate::raft::action::{AppendEntries, AppendEntriesResponse, RequestVote, RequestVoteResponse};
use crate::raft::log::LogEntry;
use crate::raft::state::{IrisRaftClock, IrisRaftNodeState, IrisRaftNodeType};
use actix_web::web::Data;
use actix_web::{web, Responder};
use log::info;
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, MutexGuard};
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn post_append(
    node_state: Data<Mutex<IrisRaftNodeState>>,
    clock: Data<Mutex<IrisRaftClock>>,
    append_entries: web::Json<AppendEntries>,
) -> actix_web::Result<impl Responder> {
    let mut node_state = node_state.lock().unwrap();
    let mut clock = clock.lock().unwrap();

    match node_state.raft_node_type {
        IrisRaftNodeType::Follower => {
            if append_entries.term < node_state.term {
                let response = AppendEntriesResponse {
                    term: node_state.term,
                    success: false,
                };

                return Ok(web::Json(serde_json::json!(response)));
            }

            let node_state = handler_entries(node_state, append_entries).await;
            clock.clock = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();

            let response = AppendEntriesResponse {
                term: node_state.term,
                success: true,
            };
            Ok(web::Json(serde_json::json!(response)))
        }
        IrisRaftNodeType::Candidate => {
            if append_entries.term > node_state.term {
                node_state.term = append_entries.term;
                node_state.raft_node_type = IrisRaftNodeType::Follower;
                node_state.leader_endpoint = Some(append_entries.leader_endpoint.clone());

                clock.clock = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis();

                let response = RequestVoteResponse {
                    term: node_state.term,
                    vote_granted: false,
                };

                return Ok(web::Json(serde_json::json!(response)));
            }

            let response = AppendEntriesResponse {
                term: node_state.term,
                success: false,
            };

            Ok(web::Json(serde_json::json!(response)))
        }
        IrisRaftNodeType::Leader => {
            if append_entries.term > node_state.term {
                node_state.term = append_entries.term;
                node_state.raft_node_type = IrisRaftNodeType::Follower;
                node_state.leader_endpoint = Some(append_entries.leader_endpoint.clone());

                clock.clock = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis();

                let response = RequestVoteResponse {
                    term: node_state.term,
                    vote_granted: false,
                };

                return Ok(web::Json(serde_json::json!(response)));
            }

            // handler append entries
            let node_state = handler_entries(node_state, append_entries).await;

            let response = AppendEntriesResponse {
                term: node_state.term,
                success: true,
            };
            Ok(web::Json(serde_json::json!(response)))
        }
    }
}

pub async fn post_commit(
    node_state: Data<Mutex<IrisRaftNodeState>>,
    log_entries: web::Json<Vec<LogEntry>>,
) -> actix_web::Result<impl Responder> {
    let node_state = node_state.lock().unwrap();

    // ignore index and term from log entries
    let append_entries = AppendEntries {
        term: node_state.term,
        leader_endpoint: node_state.node.endpoint.clone(),
        prev_log_index: node_state.last_applied_index,
        prev_log_term: 0,
        entries: log_entries.0,
        leader_commit_index: node_state.commit_index,
    };

    match node_state.raft_node_type {
        IrisRaftNodeType::Leader => {
            // handler append entries
            let node_state = handler_entries(node_state, web::Json(append_entries.clone())).await;

            // send append entries
            for node in &node_state.nodes {
                let client = reqwest::Client::new();
                client
                    .post(format!("{}/append-entries", node.endpoint))
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&append_entries)?)
                    .send()
                    .await
                    .unwrap();
            }
        }
        IrisRaftNodeType::Follower => {
            if node_state.leader_endpoint.is_none() {
                return Ok(web::Json({}));
            }

            // send append entries to leader
            let client = reqwest::Client::new();
            client
                .post(format!(
                    "{}/append-entries",
                    node_state.leader_endpoint.clone().unwrap()
                ))
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&append_entries)?)
                .send()
                .await
                .unwrap();

            return Ok(web::Json({}));
        }
        IrisRaftNodeType::Candidate => {
            return Ok(web::Json({}));
        }
    }

    Ok(web::Json({}))
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetData {
    pub key: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataResponse {
    pub data: String,
}

pub async fn get_data(
    node_state: Data<Mutex<IrisRaftNodeState>>,
    get_data: web::Json<GetData>,
) -> actix_web::Result<impl Responder> {
    let node_state = node_state.lock().unwrap();
    let data = node_state.data.get(&get_data.key).unwrap();

    Ok(web::Json(DataResponse { data: data.clone() }))
}

pub async fn post_vote(
    node_state: Data<Mutex<IrisRaftNodeState>>,
    clock: Data<Mutex<IrisRaftClock>>,
    vote_request: web::Json<RequestVote>,
) -> actix_web::Result<impl Responder> {
    let mut node_state = node_state.lock().unwrap();
    let mut clock = clock.lock().unwrap();

    match node_state.raft_node_type {
        IrisRaftNodeType::Leader => {
            if vote_request.term > node_state.term {
                node_state.raft_node_type = IrisRaftNodeType::Follower;
                node_state.term = vote_request.term;
                node_state.leader_endpoint = Some(vote_request.node.endpoint.clone());
                clock.clock = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis();

                info!(
                    "node endpoint:{} get vote from node endpoint:{} but vote request term is bigger, to Follower, clock time: {}",
                    node_state.node.endpoint, vote_request.node.endpoint, clock.clock
                );

                let response = RequestVoteResponse {
                    term: node_state.term,
                    vote_granted: true,
                };

                return Ok(web::Json(serde_json::json!(response)));
            }

            info!(
                "node endpoint:{} get vote from node endpoint:{} and reject it, clock time: {}",
                node_state.node.endpoint, vote_request.node.endpoint, clock.clock
            );

            let response = RequestVoteResponse {
                term: 0,
                vote_granted: false,
            };

            return Ok(web::Json(serde_json::json!(response)));
        },
        _ => { },
    }

    // the node is not in the cluster
    if node_state
        .nodes
        .iter()
        .map(|node| node.endpoint.clone())
        .filter(|endpoint| endpoint == &vote_request.node.endpoint)
        .count()
        == 0
    {
        let response = RequestVoteResponse {
            term: 0,
            vote_granted: false,
        };

        return Ok(web::Json(serde_json::json!(response)));
    }

    // the request term is less than the current term
    if vote_request.term < node_state.term {
        let response = RequestVoteResponse {
            term: node_state.term,
            vote_granted: false,
        };

        return Ok(web::Json(serde_json::json!(response)));
    }

    // check log index, if the request log index is less than the current log index
    // the node maybe too old.
    if vote_request.last_log_index < node_state.last_applied_index {
        let response = RequestVoteResponse {
            term: node_state.term,
            vote_granted: false,
        };

        return Ok(web::Json(serde_json::json!(response)));
    }

    let response = RequestVoteResponse {
        term: node_state.term,
        vote_granted: true,
    };

    info!(
        "node endpoint:{} get vote from node endpoint:{} and vote it, clock time: {}",
        node_state.node.endpoint, vote_request.node.endpoint, clock.clock
    );

    Ok(web::Json(serde_json::json!(response)))
}

pub async fn handler_entries<'a>(
    mut node_state: MutexGuard<'a, IrisRaftNodeState>,
    append_entries: web::Json<AppendEntries>,
) -> MutexGuard<'a, IrisRaftNodeState> {
    append_entries.entries.iter().for_each(|entry| match entry {
        LogEntry::LogSaveEntry(index, term, key, value) => {
            if index > &node_state.log.len() {
                node_state.log[*index] =
                    LogEntry::LogSaveEntry(*index, *term, key.clone(), value.clone());
                node_state.data.insert(key.clone(), value.clone());
            }
        }
        LogEntry::LogUpdateEntry(index, term, key, value) => {
            if index > &node_state.log.len() {
                node_state.log[*index] =
                    LogEntry::LogUpdateEntry(*index, *term, key.clone(), value.clone());
                node_state.data.insert(key.clone(), value.clone());
            }
        }
        LogEntry::LogDeleteEntry(index, term, key) => {
            if index > &node_state.log.len() {
                node_state.log[*index] = LogEntry::LogDeleteEntry(*index, *term, key.clone());
                node_state.data.remove(key);
            }
        }
    });

    node_state
}
