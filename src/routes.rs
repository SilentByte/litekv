////!
////! LiteKV -- A tiny key-value store with a simple REST API backed by SQLite.
////! Copyright (c) 2021 SilentByte <https://silentbyte.com/>
////!

use actix_web::{
    web,
    HttpRequest,
    HttpResponse,
    Responder,
};
use chrono::{
    DateTime,
    Utc,
};
use serde::{
    Deserialize,
    Serialize,
};

use crate::api::ApiError;
use crate::config::AppConfig;
use crate::repo::Repo;

pub async fn status() -> impl Responder {
    HttpResponse::NoContent()
}

#[derive(Debug, Deserialize)]
pub struct CommitInput {
    scope: String,
    key: String,
    value: serde_json::Value,
}

pub async fn commit_data(
    request: HttpRequest,
    config: web::Data<AppConfig>,
    repo: web::Data<Repo>,
    data: web::Json<CommitInput>,
) -> Result<HttpResponse, ApiError> {
    repo.commit_data(&data.scope, &data.key, &data.value, Utc::now(), None)?;
    Ok(HttpResponse::NoContent().finish())
}

#[derive(Debug, Deserialize)]
pub struct QueryInput {
    scope: String,
    key: String,
    limit: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct QueryResponse {
    value: serde_json::Value,
    created_on: DateTime<Utc>,
    expires_on: Option<DateTime<Utc>>,
}

pub async fn query_data(
    request: HttpRequest,
    config: web::Data<AppConfig>,
    repo: web::Data<Repo>,
    data: web::Json<QueryInput>,
) -> Result<HttpResponse, ApiError> {
    let result: Vec<QueryResponse> = repo
        .query_data(&data.scope, &data.key, data.limit)?
        .into_iter()
        .map(|d| QueryResponse {
            value: d.value,
            created_on: d.created_on,
            expires_on: d.expires_on,
        })
        .collect();

    Ok(HttpResponse::Ok().json(result))
}

pub async fn get_data(
    request: HttpRequest,
    web::Path((scope, key)): web::Path<(String, String)>,
    config: web::Data<AppConfig>,
    repo: web::Data<Repo>,
) -> Result<HttpResponse, ApiError> {
    let result = repo.query_data(&scope, &key, Some(1))?.first().cloned();
    if let Some(data) = result {
        Ok(HttpResponse::Ok().json(QueryResponse {
            value: data.value,
            created_on: data.created_on,
            expires_on: data.expires_on,
        }))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}
