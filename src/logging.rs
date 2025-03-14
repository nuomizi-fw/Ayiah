use std::path::Path;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{
    EnvFilter, Registry,
    fmt::{self, time::ChronoUtc},
    prelude::*,
};

use crate::config::ConfigManager;

/// Initialize the logging system based on configuration
pub fn init(config_manager: &'static ConfigManager) -> Result<(), String> {
    let config = config_manager.read();
    let log_config = &config.logging;

    // Initialize the base subscriber with filter
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new(format!(
            "ayiah={},tower_http=debug,axum::rejection=trace",
            log_config.level
        ))
    });

    // Start building the subscriber
    let subscriber = Registry::default().with(filter);

    // Create a pretty formatter for human-readable output
    let fmt_layer = fmt::layer()
        .with_target(true)
        .with_level(true)
        .with_file(true)
        .with_timer(ChronoUtc::new("%F %T".to_string()))
        .with_line_number(true)
        .with_span_events(fmt::format::FmtSpan::ACTIVE);

    if let Some(file_path) = &log_config.file_path {
        // Log to file with daily rotation
        let file_appender = setup_file_appender(file_path);
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

        // Create formatter layer with the non-blocking writer
        let layer = fmt_layer
            .with_ansi(false) // Disable colors in file output
            .with_writer(non_blocking);

        // We're intentionally dropping the guard here, which means logs might be lost
        // on application termination. For most applications, this is acceptable.
        tracing::subscriber::set_global_default(subscriber.with(layer))
            .map_err(|e| format!("Failed to set global default subscriber: {}", e))?;
    } else {
        // Log to console with colors
        tracing::subscriber::set_global_default(subscriber.with(fmt_layer))
            .map_err(|e| format!("Failed to set global default subscriber: {}", e))?;
    }

    Ok(())
}

/// Create a file appender with daily rotation
fn setup_file_appender(path: &str) -> RollingFileAppender {
    // Get directory and filename
    let directory = Path::new(path).parent().unwrap_or_else(|| Path::new("."));
    let filename = Path::new(path)
        .file_name()
        .unwrap_or_else(|| "ayiah.log".as_ref())
        .to_string_lossy();

    // Create directory if it doesn't exist
    if !directory.exists() {
        std::fs::create_dir_all(directory).expect("Failed to create log directory");
    }

    // Create rolling file appender with daily rotation
    RollingFileAppender::new(Rotation::DAILY, directory, filename.to_string())
}

/// Reload the logging configuration
pub fn reload(config_manager: &'static ConfigManager) -> Result<(), String> {
    // Simply re-initialize logging
    init(config_manager)
}
