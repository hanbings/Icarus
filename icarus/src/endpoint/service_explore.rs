use actix_web::{
    get,
    http::Error,
    post,
    web::{self, Data},
    HttpResponse,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{config::Config, message::Message};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FloraService {
    pub endpoint: String,
    pub created: u128,
    pub last_update: u128,
    pub service_name: String,
    pub instance_name: String,
}

#[get("/service")]
pub async fn explore_service_get_services(
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

    if config_state.service_explores.is_empty() {
        return Ok(HttpResponse::Ok().json(json!([])));
    }

    Ok(HttpResponse::Ok().json(Message::success()))
}
