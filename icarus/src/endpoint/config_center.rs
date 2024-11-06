use crate::{config::Config, message::Message};
use actix_web::{
    delete, get,
    http::Error,
    post,
    web::{self, Data},
    HttpResponse,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
struct CreateConfigRequest {
    key: String,
    value: String,
}

#[derive(Deserialize, Serialize)]
struct ConfigCenterEntry {
    key: String,
    value: String,
}

#[get("/config")]
async fn config_get_config(
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

    if config_state.config_centers.is_empty() {
        return Ok(HttpResponse::Ok().json(json!([])));
    }

    let config_center = config_state.config_centers.first().unwrap();
    let config_endpoint = config_center.endpoints.first().unwrap().clone();
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}{}", config_endpoint, "/config"))
        .header(
            "Authorization",
            format!("Bearer {}", config_center.clone().secret.unwrap()),
        )
        .send()
        .await
        .unwrap();

    Ok(HttpResponse::Ok().json(response.json::<HashMap<String, String>>().await.unwrap()))
}

#[post("/config")]
async fn config_create_config(
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

    Ok(HttpResponse::Ok().json(Message::success()))
}

#[get("/config/{key}")]
async fn config_get_config_key(
    config_state: Data<Config>,
    key: web::Path<String>,
    auth: BearerAuth,
) -> Result<HttpResponse, Error> {
    let token = config_state
        .tokens
        .iter()
        .find(|token| token.token == *auth.token());

    if token.is_none() {
        return Ok(HttpResponse::Unauthorized().json(Message::unauthorized()));
    }

    Ok(HttpResponse::Ok().json(Message::success()))
}

#[post("/config/{key}")]
async fn config_update_config_key(
    config_state: Data<Config>,
    key: web::Path<String>,
    auth: BearerAuth,
) -> Result<HttpResponse, Error> {
    let token = config_state
        .tokens
        .iter()
        .find(|token| token.token == *auth.token());

    if token.is_none() {
        return Ok(HttpResponse::Unauthorized().json(Message::unauthorized()));
    }

    Ok(HttpResponse::Ok().json(Message::success()))
}

#[delete("/config/{key}")]
async fn config_delete_config_key(
    config_state: Data<Config>,
    key: web::Path<String>,
    auth: BearerAuth,
) -> Result<HttpResponse, Error> {
    let token = config_state
        .tokens
        .iter()
        .find(|token| token.token == *auth.token());

    if token.is_none() {
        return Ok(HttpResponse::Unauthorized().json(Message::unauthorized()));
    }

    Ok(HttpResponse::Ok().json(Message::success()))
}
