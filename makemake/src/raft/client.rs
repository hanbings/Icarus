use crate::raft::log::LogEntry::{LogDeleteEntry, LogPopEntry, LogPushEntry};
use actix_web::rt::time;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::select;

pub struct Client {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopData {
    pub data: Option<String>,
    pub success: bool,
}

impl Client {
    pub async fn push(
        &self,
        endpoint: String,
        key: String,
        value: String,
        secret: Option<String>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let client = reqwest::Client::new();
        let entry = LogPushEntry(0, 0, key, value);

        let mut req = client.post(format!("{}/raft/data", endpoint)).json(&entry);

        if secret.is_some() {
            req = req.bearer_auth(secret.unwrap())
        }
        req.send().await
    }

    pub async fn pop(
        &self,
        endpoint: String,
        key: String,
        secret: Option<String>,
    ) -> reqwest::Result<PopData> {
        let client = reqwest::Client::new();
        let entry = LogPopEntry(0, 0, key);

        let mut req = client.post(format!("{}/raft/data", endpoint)).json(&entry);

        if secret.is_some() {
            req = req.bearer_auth(secret.unwrap())
        }

        let res = req.send().await;
        let res = match res {
            Ok(res) => res,
            Err(err) => {
                return Err(err);
            }
        };

        res.json::<PopData>().await
    }

    pub async fn update(
        &self,
        endpoint: String,
        key: String,
        value: String,
        secret: Option<String>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let client = reqwest::Client::new();
        let entry = LogPushEntry(0, 0, key, value);

        let mut req = client.post(format!("{}/raft/data", endpoint)).json(&entry);

        if secret.is_some() {
            req = req.bearer_auth(secret.unwrap())
        }
        req.send().await
    }

    pub async fn delete(
        &self,
        endpoint: String,
        key: String,
        secret: Option<String>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let client = reqwest::Client::new();
        let entry = LogDeleteEntry(0, 0, key);

        let mut req = client.post(format!("{}/raft/data", endpoint)).json(&entry);

        if secret.is_some() {
            req = req.bearer_auth(secret.unwrap())
        }
        req.send().await
    }
}

pub async fn async_clock(endpoint: String, secret: Option<String>) {
    loop {
        tokio::spawn(async_clock_task(endpoint.clone(), secret.clone()));
        time::sleep(Duration::from_millis(100)).await;
    }
}

async fn async_clock_task(endpoint: String, secret: Option<String>) {
    let req = async {
        let mut client = reqwest::Client::new().get(format!("{}/raft/check", endpoint));

        if secret.is_some() {
            client = client.bearer_auth(secret.unwrap())
        }
        client.send().await
    };

    let _result = select! {
        result = req => result,
        _ = time::sleep(Duration::from_secs(10)) => {
            return;
        }
    };
}
