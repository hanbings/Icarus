use crate::message::Message;
use actix_web::Responder;

pub async fn status() -> actix_web::Result<impl Responder> {
    let message = Message::success();

    Ok(actix_web::web::Json(message))
}
