use std::time::Instant;

use axum::{
    body::Body,
    http::{Request, Response},
    middleware::Next,
};
use tracing::info;

/// Custom request logger middleware
pub async fn logger(request: Request<Body>, next: Next) -> Response<Body> {
    // Extract request information and create owned copies of everything
    let method = request.method().clone();
    let uri_path = request.uri().path().to_string();

    // Get user agent
    let user_agent = request
        .headers()
        .get("user-agent")
        .map(|v| v.to_str().unwrap_or("-"))
        .unwrap_or("-")
        .to_string();

    // Record start time for latency calculation
    let start = Instant::now();

    // Process the request
    let response = next.run(request).await;

    // Calculate latency
    let latency = start.elapsed();
    let latency_ms = latency.as_secs_f64() * 1000.0; // Convert to milliseconds

    // Get response status
    let status = response.status();
    let status_code = status.as_u16();

    // Format colorized status based on code range
    let status_display = match status_code {
        100..=199 => format!("INFO {}", status_code), // Informational
        200..=299 => format!("OK {}", status_code),   // Success
        300..=399 => format!("REDIR {}", status_code), // Redirection
        400..=499 => format!("CLIENT ERR {}", status_code), // Client Error
        500..=599 => format!("SERVER ERR {}", status_code), // Server Error
        _ => format!("UNKNOWN {}", status_code),
    };

    // Log in a clean, formatted way with clear labels
    info!(
        "Request: {} {} • Status: {} • Latency: {:.2}ms • User-Agent: {}",
        method, uri_path, status_display, latency_ms, user_agent
    );

    response
}
