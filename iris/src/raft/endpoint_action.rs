#![allow(dead_code)]
use actix_web::Responder;

pub async fn post_append() -> actix_web::Result<impl Responder> {
    Ok(actix_web::web::Json({}))
}
pub async fn post_commit() -> actix_web::Result<impl Responder> {
    Ok(actix_web::web::Json({}))
}
pub async fn post_vote() -> actix_web::Result<impl Responder> {
    Ok(actix_web::web::Json({}))
}
