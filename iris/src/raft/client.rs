use crate::raft::action::{AppendEntries, AppendEntriesResponse, RequestVote, RequestVoteResponse};
use crate::raft::node::IrisRaftNode;
use std::time::Duration;
use tokio::time;

pub struct IrisRaftClient {}

#[allow(unused)]
impl IrisRaftClient {
    pub fn new() -> Self {
        Self {}
    }
    pub fn send(&self, key: String, value: String) -> AppendEntriesResponse {
        AppendEntriesResponse {
            term: 0,
            success: true,
        }
    }

    pub fn find(&self, key: String) -> AppendEntriesResponse {
        AppendEntriesResponse {
            term: 0,
            success: true,
        }
    }

    pub fn delete(&self, key: String) -> AppendEntriesResponse {
        AppendEntriesResponse {
            term: 0,
            success: true,
        }
    }

    pub fn send_heartbeat(
        &self,
        append_entries: AppendEntries,
        target: Vec<IrisRaftNode>,
    ) -> AppendEntriesResponse {
        let client = reqwest::Client::new();

        AppendEntriesResponse {
            term: 0,
            success: true,
        }
    }

    pub async fn vote(
        &self,
        vote_request: RequestVote,
        target: IrisRaftNode,
    ) -> Result<RequestVoteResponse, reqwest::Error> {
        let client = reqwest::Client::new();
        client
            .post(format!("{}/vote", target.endpoint))
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&vote_request).unwrap())
            .send()
            .await?
            .json::<RequestVoteResponse>()
            .await
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
        let client = reqwest::Client::new();
        client.post(format!("{}/check", endpoint)).send().await;
    }
}
