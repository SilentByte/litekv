////!
////! LiteKV -- A tiny key-value store with a simple REST API backed by SQLite.
////! Copyright (c) 2021 SilentByte <https://silentbyte.com/>
////!

use actix_web::{
    web,
    App,
    HttpServer,
};
use litekv::config::AppConfig;
use litekv::repo::Repo;
use litekv::routes;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = AppConfig::load();
    let bind_address = config.address();
    let repo = if let Some(db) = &config.db() {
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
