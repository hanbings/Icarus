use crate::state::{
    AppState, AppendRequest, ClockState, Node, NodeType, VoteRequest, VoteResponse,
};
use actix_web::{get, post, web, Error, HttpResponse};
use log::{info, warn};
use reqwest::Client;
use serde_json::json;
use std::time::{Duration, SystemTime};
use tokio::sync::{Mutex, MutexGuard};

#[get("/check")]
async fn check(
    app: web::Data<Mutex<AppState>>,
    clock: web::Data<Mutex<ClockState>>,
) -> Result<HttpResponse, Error> {
    let mut mutex_clock = clock.lock().await;
    let mut mutex_app = app.lock().await;

    // update clock
    mutex_clock.update_clock();

    match mutex_app.node_type {
        NodeType::Follower => {
            if mutex_clock.clock > (mutex_clock.heartbeat + 3000) {
                mutex_app.set_candidate();
                mutex_clock.update_election();
            }
        }
        NodeType::Candidate => {
            if mutex_clock.clock < mutex_clock.election + 30000 {
                vote_request(mutex_app, mutex_clock).await;
            } else {
                if mutex_app.leader.is_none() {
                    mutex_app.set_candidate();
                    mutex_clock.update_election();

                    return Ok(HttpResponse::Ok().json({}));
                }
            }
        }
        NodeType::Leader => {
            for node in &mutex_app.nodes {
                if mutex_app.node == *node {
                    continue;
                }

                tokio::spawn(heartbeat_append_entries(
                    mutex_app.clone().node,
                    mutex_app.term,
                    mutex_app.index,
                    node.endpoint.clone(),
                ));
            }
        }
    }

    Ok(HttpResponse::Ok().json({}))
}

#[post("/vote")]
async fn vote(
    app: web::Data<Mutex<AppState>>,
    clock: web::Data<Mutex<ClockState>>,
    body: web::Json<VoteRequest>,
) -> Result<HttpResponse, Error> {
    info!("Received vote request");

    let mut mutex_clock = clock.lock().await;

    // update clock
    mutex_clock.clock = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let mut mutex_state = app.lock().await;
    let leader = mutex_state.leader.clone();

    if body.term > mutex_state.term {
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

#[post("/append")]
async fn append(
    app: web::Data<Mutex<AppState>>,
    clock: web::Data<Mutex<ClockState>>,
    body: web::Json<AppendRequest>,
) -> Result<HttpResponse, Error> {
    info!("Received append request");

    let mut mutex_clock = clock.lock().await;
    let mut mutex_state = app.lock().await;
    mutex_clock.heartbeat = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    match mutex_state.node_type {
        NodeType::Follower => {
            info!("Follower received append");

            if mutex_state.leader.is_none() {
                mutex_state.set_follower(body.clone().leader, body.term, body.index);
            }

            if body.value.len() > 0 {
                for value in &body.value {
                    mutex_state.data.push(value.clone());
                }
            }
        }
        NodeType::Candidate => {
            info!("Candidate received append");

            if body.term < mutex_state.term {
                mutex_state.set_follower(body.clone().leader, body.term, body.index);
            }
        }
        NodeType::Leader => {
            warn!("Leader received append");

            if body.value.len() > 0 {
                let leader = mutex_state.leader.clone();
                if leader.is_none() {
                    return Ok(HttpResponse::Ok().json({}));
                }
                let leader = leader.unwrap();

                for value in &body.value {
                    mutex_state.data.push(value.clone());
                }

                let nodes = mutex_state.nodes.clone();
                tokio::spawn(append_request(
                    leader,
                    body.term,
                    body.index,
                    nodes,
                    body.value.clone(),
                ));
            }
        }
    }

    Ok(HttpResponse::Ok().json({}))
}

#[get("/")]
async fn get_state(app: web::Data<Mutex<AppState>>) -> Result<HttpResponse, Error> {
    let mutex_app = app.lock().await;

    Ok(HttpResponse::Ok().json(AppState {
        node: mutex_app.node.clone(),
        nodes: mutex_app.nodes.clone(),
        node_type: mutex_app.node_type.clone(),
        leader: mutex_app.leader.clone(),
        term: mutex_app.term,
        index: mutex_app.index,
        data: mutex_app.data.clone(),
    }))
}

#[get("/data")]
async fn get_data(app: web::Data<Mutex<AppState>>) -> Result<HttpResponse, Error> {
    let mutex_app = app.lock().await;

    Ok(HttpResponse::Ok().json(mutex_app.data.clone()))
}

#[post("/data/push")]
async fn push_data(
    app: web::Data<Mutex<AppState>>,
    body: web::Json<Vec<String>>,
) -> Result<HttpResponse, Error> {
    let mutex_app = app.lock().await;

    let endpoint = mutex_app.leader.clone();
    if endpoint.is_none() {
        return Ok(HttpResponse::Ok().json({}));
    }
    let endpoint = endpoint.unwrap().endpoint.clone();
    let leader = mutex_app.node.clone();
    let term = mutex_app.term;
    let index = mutex_app.index;
    let req = async move {
        let client = Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();

        client
            .post(format!("{}/append", endpoint))
            .json(&AppendRequest {
                leader,
                term,
                index,
                value: body.clone(),
            })
            .send()
            .await
            .unwrap();
    };

    tokio::spawn(req);

    Ok(HttpResponse::Ok().json({}))
}

pub async fn vote_request<'a>(
    mut mutex_app: MutexGuard<'a, AppState>,
    mut clock_app: MutexGuard<'a, ClockState>,
) -> bool {
    let client = Client::builder()
        .timeout(Duration::from_secs(3))
        .build()
        .unwrap();

    let target = mutex_app.nodes.clone();
    let candidate = mutex_app.node.clone();
    let term = mutex_app.term;
    let index = mutex_app.index;

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
                info!("{:?} {} {} {}", res, approved, failed, mutex_app.term);
                if res.granted {
                    approved = approved + 1;
                } else {
                    if res.term >= mutex_app.term && res.leader.is_some() {
                        let leader = res.leader.unwrap();
                        info!(
                            "seen leader {} term {} is better",
                            leader.endpoint, res.term
                        );

                        mutex_app.set_follower(leader, res.term, res.index);
                        clock_app.update_heartbeat();
                    }
                }
            }
            Err(_) => {
                failed += 1;
            }
        }
    }

    info!("{} {} term {}", approved, failed, mutex_app.term);
    if approved > ((target.len() - 1) / 2) || failed >= (target.len() - 1) {
        info!("Elected as leader {}", candidate.endpoint);

        mutex_app.set_leader();
        clock_app.update_heartbeat();
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
            value: vec![],
        })
        .send()
        .await?
        .json::<VoteResponse>()
        .await
}

pub async fn append_request(
    leader: Node,
    term: u64,
    index: u64,
    target: Vec<Node>,
    value: Vec<String>,
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
                value: value.clone(),
            })
            .send()
            .await;

        if let Err(e) = res {
            info!("append error: {}", e);
        }
    }
}
