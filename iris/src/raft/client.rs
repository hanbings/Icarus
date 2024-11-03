use actix_web::rt::time;
use std::time::Duration;
use tokio::select;

pub struct Client {
    pub endpoint: String,
}

impl Client {
    pub fn new(endpoint: String) -> Self {
        Self {
            endpoint,
        }
    }

    pub async fn get(&self) -> Result<reqwest::Response, reqwest::Error> {
        todo!()
    }

    pub async fn save(&self, _data: String) -> Result<reqwest::Response, reqwest::Error> {
        todo!()
    }

    pub async fn update(&self, _data: String) -> Result<reqwest::Response, reqwest::Error> {
        todo!()
    }

    pub async fn delete(&self) -> Result<reqwest::Response, reqwest::Error> {
        todo!()
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
        let client = reqwest::Client::new().get(&endpoint).send().await;

        client
    };

    let _result = select! {
        result = req => result,
        _ = time::sleep(Duration::from_secs(5)) => {
            return;
        }
    };
}
