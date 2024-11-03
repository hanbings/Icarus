use crate::raft::append::{AppendRequest, AppendResponse};
use crate::raft::log::LogEntry;
use crate::raft::node::{Node, NodeClockState, NodeState, NodeType};
use actix_web::{post, web, Error, HttpResponse};
use log::{info, warn};
use serde_json::json;
use tokio::sync::Mutex;

#[post("/append")]
async fn append(
    node_state: web::Data<Mutex<NodeState>>,
    node_clock: web::Data<Mutex<NodeClockState>>,
    body: web::Json<AppendRequest>,
) -> Result<HttpResponse, Error> {
    info!("Received append request");
    let mut node_state = node_state.lock().await;
    let mut node_clock = node_clock.lock().await;

    node_clock.update_heartbeat();

    match node_state.node_type {
        NodeType::Follower => {
            info!("Follower received append");

            if node_state.leader.is_none() {
                node_state.set_follower(body.clone().leader, body.term, body.index);
            }

            if body.entries.len() > 0 {
                for value in &body.entries {
                    node_state.log.push(value.clone());

                    match value {
                        LogEntry::LogSaveEntry(_, _, key, value) => {
                            node_state.data.insert(key.clone(), value.clone());
                        }
                        LogEntry::LogUpdateEntry(_, _, key, value) => {
                            node_state.data.insert(key.clone(), value.clone());
                        }
                        LogEntry::LogDeleteEntry(_, _, key) => {
                            node_state.data.remove(key);
                        }
                    }

                    node_state.index = node_state.index + 1;
                }
            }
        }
        NodeType::Candidate => {
            info!("Candidate received append");

            if body.term < node_state.term {
                node_state.set_follower(body.clone().leader, body.term, body.index);
            }
        }
        NodeType::Leader => {
            warn!("Leader received append");

            if body.entries.len() > 0 {
                let leader = node_state.leader.clone();
                if leader.is_none() {
                    return Ok(HttpResponse::Ok().json({}));
                }
                let leader = leader.unwrap();

                for value in &body.entries {
                    node_state.log.push(value.clone());


                    match value {
                        LogEntry::LogSaveEntry(_, _, key, value) => {
                            node_state.data.insert(key.clone(), value.clone());
                        }
                        LogEntry::LogUpdateEntry(_, _, key, value) => {
                            node_state.data.insert(key.clone(), value.clone());
                        }
                        LogEntry::LogDeleteEntry(_, _, key) => {
                            node_state.data.remove(key);
                        }
                    }

                    node_state.index = node_state.index + 1;
                }

                let nodes = node_state.nodes.clone();
                tokio::spawn(append_request(
                    leader,
                    body.term,
                    body.index,
                    nodes,
                    body.entries.clone(),
                ));
            }
        }
    }

    let res = json!(AppendResponse {
        index: node_state.index,
        success: true,
    });

    Ok(HttpResponse::Ok().json(res))
}

pub async fn append_request(
    leader: Node,
    term: u64,
    index: u64,
    target: Vec<Node>,
    entries: Vec<LogEntry>,
) {
    let client = reqwest::Client::new();

    for node in &target {
        if node.endpoint == leader.endpoint {
            continue;
        }

        let res = client
            .post(format!("{}/append", node.endpoint))
            .json(&AppendRequest {
                leader: leader.clone(),
                term,
                index,
                entries: entries.clone(),
            })
            .send()
            .await;

        if let Err(err) = res {
            info!("Append request failed {}", err);
            continue;
        }

        let res = res.unwrap().json::<AppendResponse>().await;

        match res {
            Ok(res) => if !res.success {},
            Err(err) => {
                info!("Append request response parse failed {}", err);
                continue;
            }
        }
    }
}
