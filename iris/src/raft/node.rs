use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct IrisRaftNode {
    pub id: Uuid,
    pub created_by: u128,
    pub endpoint: String,
}
