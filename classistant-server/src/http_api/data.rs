use actix_web::{web, HttpResponse};
use actix_identity::Identity;
use serde::{Serialize, Deserialize};
use crate::app_api;
use crate::db::Database;
use crate::http_api::ErrorResponse;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct GetBatchRequest {
    keys: Vec<String>,
}

#[derive(Serialize, Default)]
pub struct GetBatchResponse {
    ret: HashMap<String, (Vec<u8>, Vec<u8>)>,
}

pub fn get_batch(
    id: Identity, 
    db: web::Data<Database>,
    params: web::Json<GetBatchRequest>,
) -> HttpResponse {
    let user_id = identity_user_id!(id);
    match app_api::api_191103::data_get_batch(
        &db, 
        user_id, 
        params.keys.clone()
    ) {
        Ok(ret) => {
            let mut resp = HashMap::new();
            for (k, (d, e)) in ret {
                let data_base64 = base64::encode(&d);
                let encryption_base64 = base64::encode(&e);
                resp.insert(k, [data_base64, encryption_base64]);
            }
            HttpResponse::Ok().json(resp) 
        },
        Err(err) => internal!(err)
    }
}

#[derive(Deserialize)]
pub struct ModifyBatchRequest {
    entries: HashMap<String, (String, String)>,
}

#[derive(Serialize, Default)]
pub struct ModifyBatchResponse {
    modified: Vec<String>,
}

pub fn modify_batch(
    id: Identity, 
    db: web::Data<Database>,
    params: web::Json<ModifyBatchRequest>,
) -> HttpResponse {
    let user_id = identity_user_id!(id);
    let mut entries_bytes = HashMap::new();
    for (key, (d, e)) in params.entries.iter() {
        let data_bytes = match base64::decode(d.as_bytes()) { 
            Ok(ans) => ans,
            Err(err) => return bad_request!(err) 
        };
        let encryption_bytes = match base64::decode(e.as_bytes()) { 
            Ok(ans) => ans,
            Err(err) => return bad_request!(err) 
        };
        entries_bytes.insert(key.to_string(), (data_bytes, encryption_bytes));
    }
    match app_api::api_191103::data_modify_batch(
        &db, 
        user_id, 
        entries_bytes
    ) {
        Ok(modified) => HttpResponse::Ok().json(ModifyBatchResponse { modified }),
        Err(err) => internal!(err)
    }
}

#[derive(Deserialize)]
pub struct DeleteBatchRequest {
    keys: Vec<String>,
}

#[derive(Serialize)]
pub struct DeleteBatchResponse {}

pub fn delete_batch(
    id: Identity, 
    db: web::Data<Database>,
    params: web::Json<DeleteBatchRequest>,
) -> HttpResponse {
    let user_id = identity_user_id!(id);
    match app_api::api_191103::data_delete_batch(
        &db, 
        user_id, 
        params.keys.clone()
    ) {
        Ok(modified) => HttpResponse::Ok().json(DeleteBatchResponse {}),
        Err(err) => internal!(err)
    }
}
