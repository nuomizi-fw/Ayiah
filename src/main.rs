use std::{env, net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{Extension, Router, http::header, middleware};
use tokio::net::TcpListener;
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, propagate_header::PropagateHeaderLayer,
};
use tracing::info;

use ayiah::{
    app::{config::ConfigManager, state::AppState},
    middleware::logger,
    routes,
    utils::{graceful_shutdown::shutdown_signal, logging},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config_path = env::var("AYIAH_CONFIG_PATH").map(PathBuf::from).ok();

    // Initialize config manager
    let config_manager = ConfigManager::init(config_path)?;

    // Initialize logging with configuration
    // Note: we're passing the manager directly as required by the logging module
    logging::init(config_manager).map_err(|e| format!("Logging initialization error: {}", e))?;

    // Initialize application state
    let app_state = AppState::init(config_manager.clone()).await?;

    // Create application router
    let app = Router::new()
        .merge(routes::mount())
        .layer(Extension(Arc::new(app_state))) // Add AppState as Extension
        .layer(Extension(Arc::new(config_manager.clone()))) // Add ConfigManager directly for middleware
        .layer(middleware::from_fn(logger))
        .layer(CompressionLayer::new())
        .layer(PropagateHeaderLayer::new(header::HeaderName::from_static(
            "x-request-id",
        )))
        .layer(CorsLayer::permissive());

    // Get configured host and port
    let host = {
        let config = config_manager.read();
        config.server.host.clone()
    };

    let port = {
        let config = config_manager.read();
        config.server.port
    };

    // Parse host:port string into SocketAddr
    let address = format!("{}:{}", host, port)
        .parse::<SocketAddr>()
        .expect("Invalid server address configuration");

    info!("Server listening on {}", &address);

    let listener = TcpListener::bind(address).await?;

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}
