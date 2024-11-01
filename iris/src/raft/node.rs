use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct IrisRaftNode {
    pub id: String,
    pub endpoint: String,
}
