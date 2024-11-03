use crate::raft::node::{NodeClockState, NodeState};
use crate::raft::vote::{VoteRequest, VoteResponse};
use actix_web::{post, web, Error, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use log::info;
use tokio::sync::Mutex;

#[post("/raft/vote")]
async fn vote(
    node_state: web::Data<Mutex<NodeState>>,
    node_clock: web::Data<Mutex<NodeClockState>>,
    body: web::Json<VoteRequest>,
    auth: BearerAuth,
) -> Result<HttpResponse, Error> {
    let mut mutex_state = node_state.lock().await;
    let leader = mutex_state.leader.clone();

    if mutex_state.secret.is_some()
        && auth.token().to_string() != mutex_state.secret.clone().unwrap()
    {
        return Ok(HttpResponse::Ok().json(VoteResponse {
            granted: false,
            term: 0,
            index: 0,
            leader,
        }));
    }

    info!("Received vote request, {:?}", body);

    let mut node_clock = node_clock.lock().await;

    // update clock
    node_clock.update_clock();

    if body.term > mutex_state.term
        || (body.term == mutex_state.term && body.index > mutex_state.index)
    {
        mutex_state.set_follower(body.clone().candidate, body.term, body.index);

        return Ok(HttpResponse::Ok().json(VoteResponse {
            granted: true,
            term: mutex_state.term,
            index: mutex_state.index,
            leader: None,
        }));
    }

    Ok(HttpResponse::Ok().json(VoteResponse {
        granted: false,
        term: mutex_state.term,
        index: mutex_state.index,
        leader,
    }))
}
