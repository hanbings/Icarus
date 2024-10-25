use std::time::Duration;
use tokio::time;

pub struct IrisRaftClient {}

#[allow(unused)]
impl IrisRaftClient {
    pub fn new() -> Self { Self {} }
    pub fn send(&self, key: String, value: String) {}
    pub fn find(&self, key: String) {}
    pub fn delete(&self, key: String) {}

    /// Please use the tokio clock to perform this asynchronous task,
    /// which will time the request execution timeout logic
    /// to keep the system in a normal state.
    pub async fn async_clock() {
        loop {
            Self::async_clock_task().await;
            time::sleep(Duration::from_millis(100)).await;
        }
    }

    async fn async_clock_task() {}
}