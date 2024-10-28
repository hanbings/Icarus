use actix_web::Responder;
use iris_irides::message::Message;

pub async fn status() -> actix_web::Result<impl Responder> {
    let message = Message::success();

    Ok(actix_web::web::Json(message))
}
