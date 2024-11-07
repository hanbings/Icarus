use actix_web::{get, http::Error, web::Data, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{config::Config, message::Message};

#[derive(Serialize, Deserialize)]
struct DataRequest {
    value: String,
}

#[get("/message")]
pub async fn message_get_queue(
    config_state: Data<Config>,
    auth: BearerAuth,
) -> Result<HttpResponse, Error> {
    let token = config_state
        .tokens
        .iter()
        .find(|token| token.token == *auth.token());

    if token.is_none() {
        return Ok(HttpResponse::Unauthorized().json(Message::unauthorized()));
    }

    if config_state.message_queues.is_empty() {
        return Ok(HttpResponse::Ok().json(json!([])));
    }

    let message_queue = config_state.message_queues.first().unwrap();
    let message_queue_endpoint = message_queue.endpoints.first().unwrap().clone();
    let client = reqwest::Client::new();

    let req = client
        .get(format!("{}/message", message_queue_endpoint))
        .header(
            "Authorization",
            format!("Bearer {}", message_queue.clone().secret.unwrap()),
        )
        .send()
        .await;

    Ok(HttpResponse::Ok().json(req.unwrap().json::<DataRequest>().await.unwrap()))
}
