use crate::raft::log::LogEntry::{LogDeleteEntry, LogSaveEntry};
use actix_web::rt::time;
use std::time::Duration;
use tokio::select;

pub struct Client {}

impl Client {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn save(
        &self,
        endpoint: String,
        key: String,
        value: String,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let client = reqwest::Client::new();
        let entry = LogSaveEntry(0, 0, key, value);

        client
            .post(format!("{}/raft/data", endpoint))
            .json(&entry)
            .send()
            .await
    }

    pub async fn update(
        &self,
        endpoint: String,
        key: String,
        value: String,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let client = reqwest::Client::new();
        let entry = LogSaveEntry(0, 0, key, value);

        client
            .post(format!("{}/raft/data", endpoint))
            .json(&entry)
            .send()
            .await
    }

    pub async fn delete(
        &self,
        endpoint: String,
        key: String,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let client = reqwest::Client::new();
        let entry = LogDeleteEntry(0, 0, key);

        client
            .post(format!("{}/raft/data", endpoint))
            .json(&entry)
            .send()
            .await
    }
}

pub async fn async_clock(endpoint: String) {
    loop {
        tokio::spawn(async_clock_task(endpoint.clone()));
        time::sleep(Duration::from_millis(100)).await;
    }
}

async fn async_clock_task(endpoint: String) {
    let req = async {
        let client = reqwest::Client::new()
            .get(format!("{}/raft/check", endpoint))
            .send()
            .await;

        client
    };

    let _result = select! {
        result = req => result,
        _ = time::sleep(Duration::from_secs(10)) => {
            return;
        }
    };
}
