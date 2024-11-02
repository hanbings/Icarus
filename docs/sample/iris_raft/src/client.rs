use actix_web::rt::time;
use log::info;
use std::time::Duration;
use tokio::select;

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
        _ = time::sleep(Duration::from_secs(30)) => {
            info!("clock check timeout");
            return;
        }
    };
}
