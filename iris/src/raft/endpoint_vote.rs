use crate::raft::node::{NodeClockState, NodeState};
use crate::raft::vote::{VoteRequest, VoteResponse};
use actix_web::{post, web, Error, HttpResponse};
use log::info;
use serde_json::json;
use tokio::sync::Mutex;

#[post("/raft/vote")]
async fn vote(
    node_state: web::Data<Mutex<NodeState>>,
    node_clock: web::Data<Mutex<NodeClockState>>,
    body: web::Json<VoteRequest>,
) -> Result<HttpResponse, Error> {
    info!("Received vote request, {:?}", body);

    let mut node_clock = node_clock.lock().await;

    // update clock
    node_clock.update_clock();

    let mut mutex_state = node_state.lock().await;
    let leader = mutex_state.leader.clone();

    if body.term > mutex_state.term
        || (body.term == mutex_state.term && body.index > mutex_state.index)
    {
        mutex_state.set_follower(body.clone().candidate, body.term, body.index);

        return Ok(HttpResponse::Ok().json(json!(VoteResponse {
            granted: true,
            term: mutex_state.term,
            index: mutex_state.index,
            leader: None
        })));
    }

    Ok(HttpResponse::Ok().json(json!(VoteResponse {
        granted: false,
        term: mutex_state.term,
        index: mutex_state.index,
        leader,
    })))
}
