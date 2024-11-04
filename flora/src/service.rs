use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FloraService {
    pub endpoint: String,
    pub created: u128,
    pub last_update: u128,
    pub service_name: String,
    pub instance_name: String,
}
