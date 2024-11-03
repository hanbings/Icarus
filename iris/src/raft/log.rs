use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum LogEntry {
    // index term key value
    LogSaveEntry(usize, usize, String, String),
    // index term key value
    LogUpdateEntry(usize, usize, String, String),
    // index term key
    LogDeleteEntry(usize, usize, String),
}
