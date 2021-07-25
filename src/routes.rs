////!
////! LiteKV -- A tiny key-value store with a simple REST API backed by SQLite.
////! Copyright (c) 2021 SilentByte <https://silentbyte.com/>
////!

use actix_web::http::StatusCode;
use actix_web::{
    web,
    HttpRequest,
    HttpResponse,
    Responder,
    ResponseError,
};
use chrono::{
    DateTime,
    Utc,
};
use serde::{
    Deserialize,
    Serialize,
};

use crate::config::AppConfig;
use crate::repo::Repo;

pub async fn status() -> impl Responder {
    HttpResponse::NoContent()
}

impl std::fmt::Debug for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

#[derive(thiserror::Error)]
pub enum ApiError {
    #[error(transparent)]
    UnknownError(#[from] anyhow::Error),
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::UnknownError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
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
    let data = repo.query_data(&data.scope, &data.key)?;
    Ok(HttpResponse::Ok().json(QueryResponse {
        value: data.value,
        created_on: data.created_on,
        expires_on: data.expires_on,
    }))
}
