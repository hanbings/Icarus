#![allow(dead_code)]
use crate::raft::log::LogEntry;
use crate::raft::state::{IrisRaftClock, IrisRaftNodeState};
use bon::Builder;
use std::collections::HashMap;

#[derive(Builder)]
pub struct IrisRaftClientConfig {
    endpoint: String,
    election_timeout: (usize, usize),
    heartbeat_timeout: usize,
}

#[derive(Builder, Clone)]
pub struct IrisRaftConfig {
    pub node: Vec<String>,
    pub secret: String,
    pub endpoint: String,
    pub log_read: fn(LogEntry) -> bool,
    pub log_write: fn(LogEntry) -> bool,
    pub data_read: for<'a> fn(&'a HashMap<String, String>, &'a String) -> &'a String,
    pub data_write: fn(HashMap<String, String>, String, String) -> bool,
    pub check_log_compaction: fn(&IrisRaftNodeState, &IrisRaftClock) -> bool,
}

impl IrisRaftConfig {
    pub fn no_log_compaction(node: Vec<String>, secret: String, endpoint: String) -> Self {
        Self {
            node,
            secret,
            log_write: |_log| true,
            log_read: |_log| true,
            data_read: |data, key| {
                data.get(key).unwrap()
            },
            data_write: |mut data, key, value| {
                let result = data.insert(key, value);
                result.is_some()
            },
            check_log_compaction: |_nodes, _clock| false,
            endpoint,
        }
    }
}