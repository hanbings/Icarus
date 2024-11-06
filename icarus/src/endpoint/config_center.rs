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

#[derive(Deserialize, Serialize)]
struct CreateConfigRequest {
    key: String,
    value: String,
}

#[get("/config")]
async fn config_get_config(
    config_state: Data<Config>,
    auth: BearerAuth,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(Message::success()))
}

#[post("/config")]
async fn config_create_config(auth: BearerAuth) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(Message::success()))
}

#[get("/config/{key}")]
async fn config_get_config_key(
    key: web::Path<String>,
    auth: BearerAuth,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(Message::success()))
}

#[post("/config/{key}")]
async fn config_update_config_key(
    key: web::Path<String>,
    auth: BearerAuth,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(Message::success()))
}

#[delete("/config/{key}")]
async fn config_delete_config_key(
    key: web::Path<String>,
    auth: BearerAuth,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(Message::success()))
}
