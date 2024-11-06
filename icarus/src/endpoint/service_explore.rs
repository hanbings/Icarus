use actix_web::{get, http::Error, post, web, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::{Deserialize, Serialize};

use crate::message::Message;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FloraService {
    pub endpoint: String,
    pub created: u128,
    pub last_update: u128,
    pub service_name: String,
    pub instance_name: String,
}

#[get("/service")]
pub async fn explore_service_get_services(auth: BearerAuth) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(Message::success()))
}
