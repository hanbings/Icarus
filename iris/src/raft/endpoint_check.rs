use crate::raft::node::{Node, NodeClockState, NodeState, NodeType};
use crate::raft::vote::{VoteRequest, VoteResponse};
use actix_web::{get, web, Error, HttpResponse};
use log::info;
use reqwest::Client;
use std::time::Duration;
use tokio::sync::{Mutex, MutexGuard};
use crate::raft::append::AppendRequest;

#[get("/check")]
async fn check(
    node_state: web::Data<Mutex<NodeState>>,
    node_clock: web::Data<Mutex<NodeClockState>>,
) -> Result<HttpResponse, Error> {
    let mut node_state = node_state.lock().await;
    let mut node_clock = node_clock.lock().await;

    // update clock
    node_clock.update_clock();

    match node_state.node_type {
        NodeType::Follower => {
            if node_clock.clock > (node_clock.heartbeat + 3000) {
                node_state.set_candidate();
                node_clock.update_election();
            }
        }
        NodeType::Candidate => {
            if node_clock.clock < node_clock.election + 30000 {
                vote_request(node_state, node_clock).await;
            } else {
                if node_state.leader.is_none() {
                    node_state.set_candidate();
                    node_clock.update_election();

                    return Ok(HttpResponse::Ok().json({}));
                }
            }
        }
        NodeType::Leader => {
            for node in &node_state.nodes {
                if node_state.node == *node {
                    continue;
                }

                tokio::spawn(heartbeat_append_entries(
                    node_state.clone().node,
                    node_state.term,
                    node_state.index,
                    node.endpoint.clone(),
                ));
            }
        }
    }

    Ok(HttpResponse::Ok().json({}))
}

pub async fn vote_request<'a>(
    mut node_state: MutexGuard<'a, NodeState>,
    mut node_clock: MutexGuard<'a, NodeClockState>,
) -> bool {
    let client = Client::builder()
        .timeout(Duration::from_secs(3))
        .build()
        .unwrap();

    let target = node_state.nodes.clone();
    let candidate = node_state.node.clone();
    let term = node_state.term;
    let index = node_state.index;

    let mut approved = 0;
    let mut failed = 0;
    for node in &target {
        if node.endpoint == candidate.endpoint {
            continue;
        }

        let res = client
            .post(format!("{}/vote", node.endpoint))
            .json(&VoteRequest {
                candidate: candidate.clone(),
                term,
                index,
            })
            .send()
            .await;

        let res = match res {
            Ok(res) => res,
            Err(_) => {
                failed += 1;
                continue;
            }
        };

        match res.json::<VoteResponse>().await {
            Ok(res) => {
                info!("{:?} {} {} {}", res, approved, failed, node_state.term);
                if res.granted {
                    approved = approved + 1;
                } else {
                    if res.term >= node_state.term && res.leader.is_some() {
                        let leader = res.leader.unwrap();
                        info!(
                            "seen leader {} term {} is better",
                            leader.endpoint, res.term
                        );

                        node_state.set_follower(leader, res.term, res.index);
                        node_clock.update_heartbeat();
                    }
                }
            }
            Err(_) => {
                failed += 1;
            }
        }
    }

    info!("{} {} term {}", approved, failed, node_state.term);
    if approved > ((target.len() - 1) / 2) || failed >= (target.len() - 1) {
        info!("Elected as leader {}", candidate.endpoint);

        node_state.set_leader();
        node_clock.update_heartbeat();
    }

    failed > target.len() - 1
}

pub async fn heartbeat_append_entries(
    leader: Node,
    term: u64,
    index: u64,
    target: String,
) -> Result<VoteResponse, reqwest::Error> {
    let client = Client::builder().timeout(Duration::from_secs(3)).build()?;

    client
        .post(format!("{}/append", target))
        .json(&AppendRequest {
            leader,
            term,
            index,
            entries: vec![],
        })
        .send()
        .await?
        .json::<VoteResponse>()
        .await
}
