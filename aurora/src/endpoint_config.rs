use crate::ClientState;
use actix_web::{web::Data, Responder};
use std::sync::Mutex;

pub async fn get_config(client: Data<Mutex<ClientState>>) -> actix_web::Result<impl Responder> {
    Ok(actix_web::web::Json(
        client.try_lock().unwrap().config.clone(),
    ))
}

pub async fn get_config_by_id() -> actix_web::Result<impl Responder> {
    Ok(actix_web::web::Json({}))
}

pub async fn post_config() -> actix_web::Result<impl Responder> {
    Ok(actix_web::web::Json({}))
}
pub async fn update_config() -> actix_web::Result<impl Responder> {
    Ok(actix_web::web::Json({}))
}
pub async fn delete_config() -> actix_web::Result<impl Responder> {
    Ok(actix_web::web::Json({}))
}
