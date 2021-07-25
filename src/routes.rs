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
use chrono::Utc;
use serde::Deserialize;

use crate::config::AppConfig;
use crate::repo::Repo;

pub async fn status() -> impl Responder {
    HttpResponse::NoContent()
}

#[derive(Deserialize, Debug)]
pub struct CommitData {
    scope: String,
    key: String,
    value: serde_json::Value,
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

pub async fn commit_data(
    request: HttpRequest,
    config: web::Data<AppConfig>,
    repo: web::Data<Repo>,
    data: web::Json<CommitData>,
) -> Result<HttpResponse, ApiError> {
    repo.commit_data(&data.scope, &data.key, &data.value, Utc::now(), None)?;
    Ok(HttpResponse::NoContent().finish())
}
