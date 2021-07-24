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

pub async fn commit_data(
    request: HttpRequest,
    config: web::Data<AppConfig>,
    repo: web::Data<Repo>,
    data: web::Json<CommitData>,
) -> impl Responder {
    repo.commit_data(
        &data.scope,
        &data.key,
        &data.value.to_string(),
        Utc::now(),
        None,
    );

    HttpResponse::NoContent()
}
