#![allow(dead_code)]

use crate::raft::action::AppendEntries;
use crate::raft::client::IrisRaftClient;
use crate::raft::state::{IrisRaftClock, IrisRaftNodeState, IrisRaftNodeType};
use actix_web::web::Data;
use actix_web::Responder;
use log::{info, log};
use rand::Rng;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

/// Receive clock function calls from Iris Client, ideally triggered every 100ms.
///
/// (unverified) Because the concept of random time mechanism already exists in the raft system,
/// the delay caused by the interface call can be ignored.
pub async fn post_check(
    node_state: Data<Mutex<IrisRaftNodeState>>,
    clock: Data<Mutex<IrisRaftClock>>,
    client: Data<Mutex<IrisRaftClient>>,
) -> actix_web::Result<impl Responder> {
    let mut node_state = node_state.lock().unwrap();
    let mut clock = clock.lock().unwrap();
    let mut client = client.lock().unwrap();

    // Update the clock
    clock.clock = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    match node_state.raft_node_type {
        IrisRaftNodeType::Leader => {
            // send heartbeat
            let append_entries = AppendEntries {
                term: node_state.term,
                leader_id: node_state.node.id,
                prev_log_index: 0,
                prev_log_term: 0,
                entries: vec![],
                leader_commit_index: 0,
            };

            client.send_heartbeat(append_entries, vec![]);
        }
        IrisRaftNodeType::Candidate => {
            // If the election time exceeds the timeout period tolerated by the cluster,
            // the Candidate should become a new term
            if clock.clock > clock.last_election_time + clock.current_election_timeout_size {
                node_state.raft_node_type = IrisRaftNodeType::Candidate;
                node_state.term += 1;
                clock.last_election_time = clock.clock;
                clock.current_election_timeout_size = rand::thread_rng().gen_range(
                    node_state.config.election_timeout.0..=node_state.config.election_timeout.1,
                );

                info!(
                    "{} become Candidate, inner clock time: {}",
                    node_state.node.id, clock.clock
                );
                return Ok(actix_web::web::Json(crate::message::Message::success()));
            }
        }
        IrisRaftNodeType::Follower => {
            // If the heartbeat time exceeds the timeout period tolerated by the cluster,
            // the Leader is offline.
            if clock.clock > clock.last_heartbeat_time + node_state.config.heartbeat_timeout {
                node_state.raft_node_type = IrisRaftNodeType::Candidate;
                node_state.term += 1;
                clock.last_election_time = clock.clock;
                clock.current_election_timeout_size = rand::thread_rng().gen_range(
                    node_state.config.election_timeout.0..=node_state.config.election_timeout.1,
                );

                info!(
                    "election timeout, node id:{} become Candidate, inner clock time: {}",
                    node_state.node.id, clock.clock
                );
                return Ok(actix_web::web::Json(crate::message::Message::success()));
            }

            info!(
                "node id: {} become Follower, inner clock time: {}",
                node_state.node.id, clock.clock
            );
        }
    }

    Ok(actix_web::web::Json(crate::message::Message::success()))
}
