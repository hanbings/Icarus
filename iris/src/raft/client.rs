use crate::raft::endpoint_action::DataResponse;
use crate::raft::log::LogEntry;
use std::time::Duration;
use log::info;
use reqwest::ClientBuilder;
use tokio::net::windows::named_pipe::PipeEnd::Client;
use tokio::time;

pub struct IrisRaftClient {
    endpoint: String,
}

#[allow(unused)]
impl IrisRaftClient {
    pub fn new(endpoint: String) -> Self {
        Self { endpoint }
    }
    pub async fn save(&self, key: String, value: String) {
        let client = reqwest::Client::new();
        let entries = vec![{ LogEntry::LogSaveEntry(0, 0, key.clone(), value.clone()) }];

        let _ = client
            .post(format!("{}/commit", self.endpoint))
            .json(&serde_json::json!(entries))
            .send()
            .await;
    }

    pub async fn update(&self, key: String, value: String) {
        let client = reqwest::Client::new();
        let entries = vec![{ LogEntry::LogUpdateEntry(0, 0, key.clone(), value.clone()) }];

        let _ = client
            .post(format!("{}/commit", self.endpoint))
            .json(&serde_json::json!(entries))
            .send()
            .await;
    }

    pub async fn delete(&self, key: String) {
        let client = reqwest::Client::new();
        let entries = vec![{ LogEntry::LogDeleteEntry(0, 0, key.clone()) }];

        let _ = client
            .post(format!("{}/commit", self.endpoint))
            .json(&serde_json::json!(entries))
            .send()
            .await;
    }

    pub async fn find(&self, key: String) -> String {
        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/find", self.endpoint))
            .json(&serde_json::json!(key))
            .send()
            .await
            .unwrap();

        let data = response.json::<DataResponse>().await.unwrap();

        data.data
    }

    /// Please use the tokio clock to perform this asynchronous task,
    /// which will time the request execution timeout logic
    /// to keep the system in a normal state
    ///
    /// The time offset of this function when the client side and the server side are running
    /// on the same server at the same time is 5-20ms
    /// (which means that every 100ms it may offset to 105ms-120ms),
    /// which just meets the randomness mechanism required by the raft algorithm. :).
    ///
    /// Of course, it has not been strictly verified,
    /// for example how it works under high system load,
    /// which may be related to the asynchronous running environment.
    ///
    /// TODO: Let's modify it if we get a chance!
    pub async fn async_clock(endpoint: String) {
        loop {
            // TODO: Stupid clone()... :(
            tokio::spawn(Self::async_clock_task(endpoint.clone()));

            time::sleep(Duration::from_millis(100)).await;
        }
    }

    async fn async_clock_task(endpoint: String) {
        let client = ClientBuilder::new().timeout(Duration::from_secs(1)).build().unwrap();
        client.post(format!("{}/check", endpoint)).send().await;
    }
}
