use crate::raft::log::LogEntry::{LogDeleteEntry, LogPopEntry, LogPushEntry};
use actix_web::rt::time;
use rand::random;
use reqwest::ClientBuilder;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::select;
use tokio::time::sleep;

pub struct Client {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopData {
    pub data: String,
}

impl Client {
    pub async fn push(&self, endpoint: String, key: String, value: String, secret: Option<String>) {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(3))
            .build()
            .unwrap();

        let entry = vec![LogPushEntry(0, 0, key, value)];

        let mut req = client.post(format!("{}/raft/data", endpoint)).json(&entry);

        if secret.is_some() {
            req = req.bearer_auth(secret.unwrap())
        }

        tokio::spawn(req.send());
    }

    pub async fn pop(&self, endpoint: String, key: String, secret: Option<String>) -> String {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(3))
            .build()
            .unwrap();
        let token = random::<u128>().to_string();
        let entry = vec![LogPopEntry(0, 0, token.clone(), key)];

        let mut req = client.post(format!("{}/raft/data", endpoint)).json(&entry);

        if secret.is_some() {
            req = req.bearer_auth(secret.unwrap())
        }

        tokio::spawn(req.send());

        token
    }

    pub async fn get_pop_data(
        &self,
        endpoint: String,
        token: String,
        secret: Option<String>,
    ) -> Option<PopData> {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(3))
            .build()
            .unwrap();

        for _round in 0..5 {
            sleep(Duration::from_millis(100)).await;

            let mut req = client.get(format!("{}/raft/data/pop/{}", endpoint, token));

            if secret.is_some() {
                req = req.bearer_auth(secret.clone().unwrap())
            }

            let res = req.send().await;
            if res.is_ok() {
                let res = res.unwrap().json::<PopData>().await;
                if let Ok(res) = res {
                    return Some(res);
                }
            }
        }

        None
    }

    pub async fn update(
        &self,
        endpoint: String,
        key: String,
        value: String,
        secret: Option<String>,
    ) {
        let client = reqwest::Client::new();
        let entry = vec![LogPushEntry(0, 0, key, value)];

        let mut req = client.post(format!("{}/raft/data", endpoint)).json(&entry);

        if secret.is_some() {
            req = req.bearer_auth(secret.unwrap())
        }

        tokio::spawn(req.send());
    }

    pub async fn delete(&self, endpoint: String, key: String, secret: Option<String>) {
        let client = reqwest::Client::new();
        let entry = vec![LogDeleteEntry(0, 0, key)];

        let mut req = client.post(format!("{}/raft/data", endpoint)).json(&entry);

        if secret.is_some() {
            req = req.bearer_auth(secret.unwrap())
        }

        tokio::spawn(req.send());
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
