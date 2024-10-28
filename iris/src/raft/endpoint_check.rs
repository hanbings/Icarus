#![allow(dead_code)]

use crate::raft::state::{IrisRaftClock, IrisRaftNodeState};
use actix_web::web::Data;
use actix_web::Responder;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use log::{info, log};
use crate::raft::client::IrisRaftClient;

/// Receive clock function calls from Iris Client, ideally triggered every 100ms.
///
/// (unverified) Because the concept of random time mechanism already exists in the raft system,
/// the delay caused by the interface call can be ignored.
pub async fn post_check(
    node_state: Data<Mutex<IrisRaftNodeState>>,
    clock: Data<Mutex<IrisRaftClock>>
) -> actix_web::Result<impl Responder> {
    info!("clock: {}", SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis());

    let mut node_state = node_state.lock().unwrap();
    let mut clock = clock.lock().unwrap();

    // update heartbeat clock
    clock.heartbeat_clock += 1;

    // If the heartbeat time is 0, this timer is started.
    if clock.heartbeat_clock == 0 {

    }

    // If the heartbeat time exceeds the timeout period tolerated by the cluster,
    // the Leader is offline.
    if clock.heartbeat_clock > clock.last_heartbeat_time + node_state.config.heartbeat_timeout {

    }

    Ok(actix_web::web::Json(crate::message::Message::success()))
}