use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct IrisRaftNode {
    pub id: String,
    pub created_by: u128,
    pub endpoint: String,
}
