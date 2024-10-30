use actix_web::{web::Data, Responder};
use iris_irides::raft::state::IrisRaftNodeState;
use std::sync::Mutex;

pub async fn get_config(
    node_state: Data<Mutex<IrisRaftNodeState>>,
) -> actix_web::Result<impl Responder> {
    let node_state = node_state.try_lock().unwrap();

    Ok(actix_web::web::Json(node_state.clone().data))
}

pub async fn get_config_by_id(
    node_state: Data<Mutex<IrisRaftNodeState>>,
) -> actix_web::Result<impl Responder> {
    let _node_state = node_state.try_lock().unwrap();

    Ok(actix_web::web::Json({}))
}

pub async fn post_config(
    node_state: Data<Mutex<IrisRaftNodeState>>,
) -> actix_web::Result<impl Responder> {
    let _node_state = node_state.try_lock().unwrap();

    Ok(actix_web::web::Json({}))
}
pub async fn update_config(
    node_state: Data<Mutex<IrisRaftNodeState>>,
) -> actix_web::Result<impl Responder> {
    let _node_state = node_state.try_lock().unwrap();

    Ok(actix_web::web::Json({}))
}
pub async fn delete_config(
    node_state: Data<Mutex<IrisRaftNodeState>>,
) -> actix_web::Result<impl Responder> {
    let _node_state = node_state.try_lock().unwrap();

    Ok(actix_web::web::Json({}))
}
