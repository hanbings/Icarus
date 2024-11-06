use actix_web::{http::Error, post, web, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{config::Config, message::Message};

#[post("/login")]
pub async fn login(config: web::Data<Config>, auth: BearerAuth) -> Result<HttpResponse, Error> {
    let token = config
        .tokens
        .iter()
        .find(|token| token.token == *auth.token());

    if token.is_none() {
        return Ok(HttpResponse::Unauthorized().json(Message::unauthorized()));
    }

    Ok(HttpResponse::Ok().json(token.unwrap().clone()))
}
