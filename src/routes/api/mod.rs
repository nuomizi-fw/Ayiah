use axum::Router;

pub mod preferences;
pub mod users;

/// Mount all API routes
pub fn mount() -> Router {
    Router::new()
}
