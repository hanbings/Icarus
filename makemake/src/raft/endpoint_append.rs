use crate::message::Message;
use crate::raft::append::{AppendRequest, AppendResponse};
use crate::raft::log::LogEntry;
use crate::raft::node::{Node, NodeClockState, NodeState, NodeType};
use actix_web::{post, web, Error, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use log::info;
use reqwest::ClientBuilder;
use serde_json::json;
use std::collections::{HashMap, VecDeque};
use std::time::Duration;
use tokio::sync::Mutex;

#[post("/raft/append")]
async fn append(
    node_state: web::Data<Mutex<NodeState>>,
    pop_state: web::Data<Mutex<HashMap<String, String>>>,
    node_clock: web::Data<Mutex<NodeClockState>>,
    body: web::Json<AppendRequest>,
    auth: BearerAuth,
) -> Result<HttpResponse, Error> {
    let mut node_state = node_state.lock().await;
    let mut node_clock = node_clock.lock().await;

    if node_state.secret.is_some() && *auth.token() != node_state.secret.clone().unwrap() {
        return Ok(HttpResponse::Unauthorized().json(Message::unauthorized()));
    }

    node_clock.update_heartbeat();

    match node_state.node_type {
        NodeType::Follower => {
            if node_state.leader.is_none() {
                node_state.set_follower(body.clone().leader, body.term, body.index);
            }

            if !body.entries.is_empty() {
                for value in &body.entries {
                    node_state.log.push(value.clone());

                    match value {
                        LogEntry::LogPushEntry(_, _, key, value) => {
                            if node_state.data.contains_key(key) {
                                let deque = node_state.data.get_mut(key).unwrap();
                                deque.push_back(value.clone());
                            } else {
                                let mut deque = VecDeque::new();
                                deque.push_back(value.clone());
                                node_state.data.insert(key.clone(), deque);
                            }
                        }
                        LogEntry::LogPopEntry(_, _, _token, key) => {
                            let deque = node_state.data.get_mut(key).unwrap();
                            if !deque.is_empty() {
                                let _ = deque.pop_front();
                            }
                        }
                        LogEntry::LogUpdateEntry(_, _, key, value) => {
                            let mut deque = VecDeque::new();
                            deque.push_back(value.clone());
                            node_state.data.insert(key.clone(), deque);
                        }
                        LogEntry::LogDeleteEntry(_, _, key) => {
                            node_state.data.remove(key);
                        }
                    }

                    node_state.index += 1;
                }
            }
        }
        NodeType::Candidate => {
            if body.term < node_state.term {
                node_state.set_follower(body.clone().leader, body.term, body.index);
            }
        }
        NodeType::Leader => {
            if !body.entries.is_empty() {
                let leader = node_state.leader.clone();
                if leader.is_none() {
                    return Ok(HttpResponse::Ok().json(Message::fail()));
                }
                let leader = leader.unwrap();

                for value in &body.entries {
                    node_state.log.push(value.clone());

                    match value {
                        LogEntry::LogPushEntry(_, _, key, value) => {
                            if node_state.data.contains_key(key) {
                                let deque = node_state.data.get_mut(key).unwrap();
                                deque.push_back(value.clone());
                            } else {
                                let mut deque = VecDeque::new();
                                deque.push_back(value.clone());
                                node_state.data.insert(key.clone(), deque);
                            }
                        }
                        LogEntry::LogPopEntry(_, _, token, key) => {
                            let deque = node_state.data.get_mut(key).unwrap();
                            if !deque.is_empty() {
                                let data = deque.pop_front();

                                if data.is_some() {
                                    let pop_data = data.unwrap();
                                    let mut pop_state = pop_state.lock().await;
                                    pop_state.insert(token.to_string(), pop_data);
                                }
                            }
                        }
                        LogEntry::LogUpdateEntry(_, _, key, value) => {
                            let mut deque = VecDeque::new();
                            deque.push_back(value.clone());
                            node_state.data.insert(key.clone(), deque);
                        }
                        LogEntry::LogDeleteEntry(_, _, key) => {
                            node_state.data.remove(key);
                        }
                    }

                    node_state.index += 1;
                }

                let nodes = node_state.nodes.clone();
                tokio::spawn(append_request(
                    leader,
                    body.term,
                    body.index,
                    nodes,
                    body.entries.clone(),
                    node_state.secret.clone(),
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
    secret: std::option::Option<String>,
) {
    let client = ClientBuilder::new()
        .timeout(Duration::from_secs(3))
        .build()
        .unwrap();

    for node in &target {
        if node.endpoint == leader.endpoint {
            continue;
        }

        let mut req = client
            .post(format!("{}/raft/append", node.endpoint))
            .json(&AppendRequest {
                leader: leader.clone(),
                term,
                index,
                entries: entries.clone(),
            });

        if secret.is_some() {
            req = req.bearer_auth(secret.clone().unwrap());
        }

        let res = req.send().await;

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
