#![allow(dead_code)]

use crate::raft::action::{RequestVote, RequestVoteResponse};
use crate::raft::state::{IrisRaftClock, IrisRaftNodeState};
use actix_web::web::Data;
use actix_web::{web, Responder};
use log::info;
use std::sync::Mutex;

pub async fn post_append() -> actix_web::Result<impl Responder> {
    Ok(actix_web::web::Json({}))
}
pub async fn post_commit() -> actix_web::Result<impl Responder> {
    Ok(actix_web::web::Json({}))
}
pub async fn post_vote(
    node_state: Data<Mutex<IrisRaftNodeState>>,
    clock: Data<Mutex<IrisRaftClock>>,
    vote_request: web::Json<RequestVote>,
) -> actix_web::Result<impl Responder> {
    let mut node_state = node_state.lock().unwrap();
    let mut clock = clock.lock().unwrap();

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
        "node id:{} get vote from node id:{} and vote it, clock time: {}",
        node_state.node.id, vote_request.node.id, clock.clock
    );

    Ok(web::Json(serde_json::json!(response)))
}
