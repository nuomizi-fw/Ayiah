use std::path::PathBuf;
use std::{env, net::SocketAddr};

use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;

use ayiah::{config::ConfigManager, graceful_shutdown::shutdown_signal, logging};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config_path = env::var("AYIAH_CONFIG_PATH").map(PathBuf::from).ok();

    // Initialize config manager
    let config_manager = ConfigManager::init(config_path)?;

    // Initialize logging with configuration
    logging::init(config_manager).map_err(|e| format!("Logging initialization error: {}", e))?;

    // Create CORS layer
    let cors_origins = {
        let config = config_manager.read();
        config.server.cors_origins.clone()
    };

    let cors = if cors_origins.iter().any(|origin| origin == "*") {
        CorsLayer::new().allow_origin(Any)
    } else {
        let origins = cors_origins
            .iter()
            .filter_map(|origin| origin.parse().ok())
            .collect::<Vec<_>>();
        CorsLayer::new().allow_origin(origins)
    }
    .allow_methods(Any)
    .allow_headers(Any);

    // Create application router
    let app = Router::new()
        .route("/", get(health_check))
        .layer(cors)
        .layer(TraceLayer::new_for_http());

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

async fn health_check() -> &'static str {
    "ok"
}
