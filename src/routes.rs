////!
////! LiteKV -- A tiny key-value store with a simple REST API backed by SQLite.
////! Copyright (c) 2021 SilentByte <https://silentbyte.com/>
////!

use actix_web::{
    HttpResponse,
    Responder,
};

pub async fn status() -> impl Responder {
    HttpResponse::NoContent()
}
