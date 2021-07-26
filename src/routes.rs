////!
////! LiteKV -- A tiny key-value store with a simple REST API backed by SQLite.
////! Copyright (c) 2021 SilentByte <https://silentbyte.com/>
////!

use actix_web::{
    web,
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
pub struct ValueInput {
    scope: String,
    key: String,
    value: serde_json::Value,
    expires_on: Option<DateTime<Utc>>,
}

pub async fn commit_multiple_data(
    config: web::Data<AppConfig>,
    repo: web::Data<Repo>,
    data: web::Json<Vec<ValueInput>>,
) -> Result<HttpResponse, ApiError> {
    if config.readonly() {
        return Err(ApiError::ReadonlyDataStore);
    }

    let values: Vec<crate::repo::Value> = data
        .into_inner()
        .into_iter()
        .map(|v| crate::repo::Value {
            scope: v.scope,
            key: v.key,
            value: v.value,
            created_on: Utc::now(),
            expires_on: v.expires_on,
        })
        .collect();

    repo.commit_values(&values)?;
    Ok(HttpResponse::NoContent().finish())
}

#[derive(Debug, Deserialize)]
pub struct QueryInput {
    scope: String,
    key: String,
    start_on: Option<DateTime<Utc>>,
    end_on: Option<DateTime<Utc>>,
    limit: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct QueryResponse {
    value: serde_json::Value,
    created_on: DateTime<Utc>,
    expires_on: Option<DateTime<Utc>>,
}

pub async fn query_data(
    repo: web::Data<Repo>,
    data: web::Json<QueryInput>,
) -> Result<HttpResponse, ApiError> {
    let result: Vec<QueryResponse> = repo
        .query_values(
            &data.scope,
            &data.key,
            data.start_on,
            data.end_on,
            data.limit,
        )?
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
    web::Path((scope, key)): web::Path<(String, String)>,
    repo: web::Data<Repo>,
) -> Result<HttpResponse, ApiError> {
    let result = repo
        .query_values(&scope, &key, None, None, Some(1))?
        .first()
        .cloned();
    if let Some(data) = result {
        Ok(HttpResponse::Ok().json(QueryResponse {
            value: data.value,
            created_on: data.created_on,
            expires_on: data.expires_on,
        }))
    } else {
        Err(ApiError::ValueNotFound(key))
    }
}
