use serde::{Deserialize, Serialize};

#[allow(clippy::enum_variant_names)]
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum LogEntry {
    // index term key value
    LogPushEntry(usize, usize, String, String),
    // index term token key
    LogPopEntry(usize, usize, String, String),
    // index term key value
    LogUpdateEntry(usize, usize, String, String),
    // index term key
    LogDeleteEntry(usize, usize, String),
}
