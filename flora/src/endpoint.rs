use crate::service::FloraService;
use actix_web::{delete, get, post, web, Error, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use iris_irides::message::Message;
use iris_irides::raft::client::Client;
use iris_irides::raft::node::NodeState;
use tokio::sync::Mutex;

#[get("/explore/{service_name}")]
pub async fn explore_service(
    node_state: web::Data<Mutex<NodeState>>,
    service_name: web::Data<String>,
    auth: BearerAuth,
) -> Result<HttpResponse, Error> {
    let node_state = node_state.lock().await;

    if node_state.secret.is_some() && *auth.token() != node_state.secret.clone().unwrap() {
        return Ok(HttpResponse::Unauthorized().json(Message::unauthorized()));
    }

    let services: Vec<FloraService> = node_state
        .data
        .iter()
        .map(|instance| serde_json::from_str::<FloraService>(instance.1))
        .filter(|service| service.is_ok())
        .map(|service| service.unwrap())
        .filter(|service| {
            service.service_name.as_str() == service_name.clone().into_inner().as_str()
        })
        .collect();

    Ok(HttpResponse::Unauthorized().json(services))
}

#[get("/service")]
pub async fn get_services(
    node_state: web::Data<Mutex<NodeState>>,
    _client: web::Data<Mutex<Client>>,
    auth: BearerAuth,
) -> Result<HttpResponse, Error> {
    let node_state = node_state.lock().await;

    if node_state.secret.is_some() && *auth.token() != node_state.secret.clone().unwrap() {
        return Ok(HttpResponse::Unauthorized().json(Message::unauthorized()));
    }

    Ok(HttpResponse::Unauthorized().json(node_state.data.clone()))
}

#[post("/service")]
pub async fn post_service(
    node_state: web::Data<Mutex<NodeState>>,
    client: web::Data<Mutex<Client>>,
    body: web::Json<FloraService>,
    auth: BearerAuth,
) -> Result<HttpResponse, Error> {
    let node_state = node_state.lock().await;

    if node_state.secret.is_some() && *auth.token() != node_state.secret.clone().unwrap() {
        return Ok(HttpResponse::Unauthorized().json(Message::unauthorized()));
    }

    let leader = node_state.leader.clone();
    if leader.is_none() {
        return Ok(HttpResponse::Ok().json(Message::fail()));
    }
    let client = client.lock().await;
    let body = body.into_inner();
    client
        .save(
            leader.unwrap().endpoint.clone(),
            body.instance_name.clone(),
            serde_json::to_string(&body)?,
            node_state.secret.clone(),
        )
        .await
        .unwrap();

    Ok(HttpResponse::Ok().json(Message::success()))
}

#[get("/service/{instance_name}")]
pub async fn get_instance(
    node_state: web::Data<Mutex<NodeState>>,
    _client: web::Data<Mutex<Client>>,
    instance_name: web::Path<String>,
    auth: BearerAuth,
) -> Result<HttpResponse, Error> {
    let node_state = node_state.lock().await;

    if node_state.secret.is_some() && *auth.token() != node_state.secret.clone().unwrap() {
        return Ok(HttpResponse::Unauthorized().json(Message::unauthorized()));
    }

    let service = node_state.data.get(&instance_name.into_inner());
    if service.is_none() {
        return Ok(HttpResponse::Unauthorized().json(Message::fail()));
    }

    Ok(HttpResponse::Ok().json(Message::success()))
}

#[post("/service/{instance_name}")]
pub async fn update_instance(
    node_state: web::Data<Mutex<NodeState>>,
    client: web::Data<Mutex<Client>>,
    body: web::Json<FloraService>,
    auth: BearerAuth,
) -> Result<HttpResponse, Error> {
    let node_state = node_state.lock().await;

    if node_state.secret.is_some() && *auth.token() != node_state.secret.clone().unwrap() {
        return Ok(HttpResponse::Unauthorized().json(Message::unauthorized()));
    }

    let leader = node_state.leader.clone();
    if leader.is_none() {
        return Ok(HttpResponse::Ok().json(Message::fail()));
    }
    let body = body.into_inner();
    let client = client.lock().await;
    client
        .update(
            leader.unwrap().endpoint.clone(),
            body.clone().instance_name,
            serde_json::to_string(&body)?,
            node_state.secret.clone(),
        )
        .await
        .unwrap();

    Ok(HttpResponse::Ok().json(Message::success()))
}

#[delete("/service/{instance_name}")]
pub async fn delete_instance(
    node_state: web::Data<Mutex<NodeState>>,
    client: web::Data<Mutex<Client>>,
    instance_name: web::Path<String>,
    auth: BearerAuth,
) -> Result<HttpResponse, Error> {
    let node_state = node_state.lock().await;

    if node_state.secret.is_some() && *auth.token() != node_state.secret.clone().unwrap() {
        return Ok(HttpResponse::Unauthorized().json(Message::unauthorized()));
    }

    let leader = node_state.leader.clone();
    if leader.is_none() {
        return Ok(HttpResponse::Ok().json(Message::fail()));
    }
    let client = client.lock().await;
    client
        .delete(
            leader.unwrap().endpoint.clone(),
            instance_name.into_inner(),
            node_state.secret.clone(),
        )
        .await
        .unwrap();

    Ok(HttpResponse::Ok().json(Message::success()))
}
