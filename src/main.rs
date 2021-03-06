////!
////! LiteKV -- A tiny key-value store with a simple REST API backed by SQLite.
////! Copyright (c) 2021 SilentByte <https://silentbyte.com/>
////!

use actix_web::middleware::Logger;
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
        web::Data::new(Repo::create_with_file(db, config.readonly())?)
    } else {
        web::Data::new(Repo::create_in_memory()?)
    };

    HttpServer::new(move || {
        App::new()
            .data(config.clone())
            .app_data(repo.clone())
            .wrap(Logger::default())
            .service(web::resource("/status").route(web::get().to(routes::status)))
            .service(web::resource("/commit").route(web::post().to(routes::commit_multiple_data)))
            .service(web::resource("/query").route(web::post().to(routes::query_data)))
            .service(web::resource("/get/{scope}/{key}").route(web::get().to(routes::get_data)))
    })
    .bind(bind_address)?
    .run()
    .await?;

    Ok(())
}
