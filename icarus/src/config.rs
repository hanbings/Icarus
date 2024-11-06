use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub ip: String,
    pub port: u16,
    pub tokens: Vec<TokenConfigEntry>,
    pub config_centers: Vec<DistributedConfigEntry>,
    pub service_explores: Vec<DistributedConfigEntry>,
    pub message_queues: Vec<DistributedConfigEntry>,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct TokenConfigEntry {
    pub name: String,
    pub token: String,
    pub allowed: Vec<String>,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct DistributedConfigEntry {
    pub name: String,
    pub secret: Option<String>,
    pub endpoints: Vec<String>,
    pub size: u32,
}
