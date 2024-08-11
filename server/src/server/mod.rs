use actix_cors::Cors;
use actix_route_config::Routable;
use actix_web::{App, HttpServer};
use noiseless_tracing_actix_web::NoiselessRootSpanBuilder;
use tokio::sync::RwLock;

use crate::file::{AppConfig, AppStorage};
use crate::server::types::{MutAppStorage, WConfig, WStorage};

mod routes;
mod types;

pub async fn run_server(config: AppConfig, storage: AppStorage) -> color_eyre::Result<()> {
    let port = config.server.port;

    let storage = WStorage::new(MutAppStorage(RwLock::new(storage)));
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(tracing_actix_web::TracingLogger::<NoiselessRootSpanBuilder>::new())
            .app_data(WConfig::new(config.clone()))
            .app_data(storage.clone())
            .configure(routes::Router::configure)
    })
    .bind(format!("[::]:{port}"))?
    .bind(format!("0.0.0.0:{port}"))?
    .run()
    .await?;

    Ok(())
}
