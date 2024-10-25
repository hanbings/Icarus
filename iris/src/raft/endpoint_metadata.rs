#![allow(dead_code)]
use actix_web::Responder;

pub async fn get_node() -> actix_web::Result<impl Responder> { Ok(actix_web::web::Json({})) }
pub async fn get_nodes() -> actix_web::Result<impl Responder> { Ok(actix_web::web::Json({})) }
pub async fn get_status() -> actix_web::Result<impl Responder> { Ok(actix_web::web::Json({})) }