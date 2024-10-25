#![allow(dead_code)]
use actix_web::Responder;

pub async fn post_check() -> actix_web::Result<impl Responder> { Ok(actix_web::web::Json({})) }