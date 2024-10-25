use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum LogEntry {
    LogSaveEntry,
    LogUpdateEntry,
    LogDeleteEntry,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LogSaveEntry {
    key: String,
    value: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LogUpdateEntry {
    key: String,
    value: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LogDeleteEntry {
    key: String,
}