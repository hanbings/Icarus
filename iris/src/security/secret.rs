use actix_web::dev::ServiceRequest;
use actix_web::{error, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;

pub async fn secret_middleware(
    req: ServiceRequest,
    credentials: Option<BearerAuth>,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let Some(_credentials) = credentials else {
        return Err((error::ErrorUnauthorized("Unauthorized"), req));
    };

    Ok(req)
}
