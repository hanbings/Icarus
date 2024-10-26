use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub ip: String,
    pub port: u16,
    pub endpoint: String,
    pub node: Vec<String>,
    pub secret: String,
}
