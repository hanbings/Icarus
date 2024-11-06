use actix_web::{get, http::Error, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::message::Message;

#[get("/message")]
pub async fn message_get_queue(auth: BearerAuth) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(Message::success()))
}
