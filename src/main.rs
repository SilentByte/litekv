////!
////! LiteKV -- A tiny key-value store with a simple REST API backed by SQLite.
////! Copyright (c) 2021 SilentByte <https://silentbyte.com/>
////!

use actix_web::{
    web,
    App,
    HttpServer,
};
use argh::FromArgs;
use litekv::repo::Repo;
use litekv::routes;

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

/// Reach new heights.
#[derive(Debug, Clone, FromArgs)]
struct AppConfig {
    /// host name or address for which LiteKV is listening
    #[argh(option, default = "\"127.0.0.1\".to_string()")]
    host: String,

    /// port on which LiteKV is listening
    #[argh(option, default = "8088")]
    port: usize,

    /// path to the database file
    #[argh(option)]
    db: Option<String>,
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config: AppConfig = argh::from_env();
    let bind_address = format!("{}:{}", config.host, config.port);
    let repo = if let Some(db) = &config.db {
        web::Data::new(Repo::create_with_file(db)?)
    } else {
        web::Data::new(Repo::create_in_memory()?)
    };

    HttpServer::new(move || {
        App::new()
            .data(config.clone())
            .data(repo.clone())
            .service(web::resource("/status").route(web::get().to(routes::status)))
    })
    .bind(bind_address)?
    .run()
    .await?;

    Ok(())
}
