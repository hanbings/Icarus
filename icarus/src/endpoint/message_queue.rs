use actix_web::{get, http::Error, web::Data, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde_json::json;

use crate::{config::Config, message::Message};

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

    Ok(HttpResponse::Ok().json(Message::success()))
}
