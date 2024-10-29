use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum LogEntry {
    LogSaveEntry,
    LogUpdateEntry,
    LogDeleteEntry,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LogSaveEntry {
    index: usize,
    term: usize,
    key: String,
    value: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LogUpdateEntry {
    index: usize,
    term: usize,
    key: String,
    value: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LogDeleteEntry {
    index: usize,
    term: usize,
    key: String,
}
