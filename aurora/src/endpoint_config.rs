use actix_web::{web::Data, Responder};
use std::collections::HashMap;
use std::sync::Mutex;

pub async fn get_config(
    client: Data<Mutex<HashMap<String, String>>>,
) -> actix_web::Result<impl Responder> {
    Ok(actix_web::web::Json(client.try_lock().unwrap().clone()))
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
