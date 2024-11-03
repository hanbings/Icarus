use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::net::IpAddr;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct GossipNode {
    pub id: Uuid,
    pub host: IpAddr,
    pub port: u16,
    pub created_by: u128,
}

impl GossipNode {
    pub fn default_node() -> Self {
        let local_ip = {
            let socket = std::net::UdpSocket::bind("0.0.0.0:0").unwrap();
            socket.connect("8.8.8.8:80").unwrap();
            (
                socket.local_addr().unwrap().ip(),
                socket.local_addr().unwrap().port(),
            )
        };

        GossipNode {
            id: Uuid::new_v4(),
            host: local_ip.0,
            port: local_ip.1,
            created_by: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AntiEntropyType {
    Push,
    Pull,
    PushAndPull,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AntiEntropyData {
    AntiEntropyParting(String, String),
    AntiEntropyFull(HashMap<String, String>),
    AntiEntropyNeighborNodesSync(HashSet<Uuid>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AntiEntropyMessage {
    origin_id: Uuid,
    target_id: Uuid,
    created_by: u64,
    anti_entropy_type: AntiEntropyType,
    data: AntiEntropyData,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RumorMongeringData {
    Save(String, String),
    Update(String, String),
    Delete(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RumorMongeringMessage {
    origin_id: Uuid,
    target_id: HashSet<Uuid>,
    created_by: u64,
    data: RumorMongeringData,
}
