use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub message: String,
    pub code: i32,
}

impl Message {
    pub fn success() -> Self {
        Message {
            message: "success".to_string(),
            code: 200,
        }
    }

    pub fn fail() -> Self {
        Message {
            message: "fail".to_string(),
            code: 500,
        }
    }

    pub fn unauthorized() -> Self {
        Message {
            message: "unauthorized".to_string(),
            code: 401,
        }
    }
}
