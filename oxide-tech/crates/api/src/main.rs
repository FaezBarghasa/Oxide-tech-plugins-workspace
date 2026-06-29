mod config;
mod handlers;
mod middleware;
mod models;
mod routes;
mod services;

use actix_web::{web, App, HttpServer, middleware::Logger};
use config::AppConfig;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::INFO)
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    let config = AppConfig::from_env().expect("Failed to load config");
    let config = Arc::new(config);

    // Initialize database
    let db_url = &config.surrealdb_url;
    let db = surrealdb::engine::any::connect(db_url)
        .await
        .expect("Failed to connect to SurrealDB");

    tracing::info!("✓ Connected to SurrealDB");

    // Initialize HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(db.clone()))
            .wrap(Logger::default())
            .wrap(middleware::MetricsMiddleware)
            .configure(routes::configure)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
