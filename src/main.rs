use std::{env, path::PathBuf, sync::Arc};

use axum::{Extension, Router, http::HeaderName, middleware};
use migration::{Migrator, MigratorTrait};
use tokio::net::TcpListener;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    propagate_header::PropagateHeaderLayer,
    request_id::{MakeRequestUuid, SetRequestIdLayer},
    services::{ServeDir, ServeFile},
};
use tracing::info;

use ayiah::{
    Context,
    app::config::ConfigManager,
    db::{self},
    middleware::logger as middleware_logger,
    routes,
    utils::{graceful_shutdown::shutdown_signal, logger},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config_path = env::var("AYIAH_CONFIG_PATH").map(PathBuf::from).ok();

    // Initialize config manager
    let config_manager = ConfigManager::init(config_path)?;

    // Initialize logging with configuration
    // Note: we're passing the manager directly as required by the logging module
    logger::init(config_manager).map_err(|e| format!("Logging initialization error: {}", e))?;

    // Connect to database
    let conn = db::init().await?;

    // Migrate database
    Migrator::up(&conn, None).await.unwrap();

    // Create application router
    let app = Router::new()
        .merge(routes::mount())
        .fallback_service(
            ServeDir::new("/dist").not_found_service(ServeFile::new("/dist/index.html")),
        )
        .layer(Extension(Arc::new(Context {
            db: conn,
            config: config_manager.clone(),
        })))
        .layer(middleware::from_fn(middleware_logger))
        .layer(CompressionLayer::new())
        .layer(PropagateHeaderLayer::new(HeaderName::from_static(
            "x-request-id",
        )))
        .layer(SetRequestIdLayer::new(
            HeaderName::from_static("x-request-id"),
            MakeRequestUuid,
        ))
        .layer(CorsLayer::permissive());

    // Parse host:port string into SocketAddr
    let address = config_manager.socket_addr()?;

    info!("Server listening on {}", &address);

    let listener = TcpListener::bind(address).await?;

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}
