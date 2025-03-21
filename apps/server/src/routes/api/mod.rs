use axum::Router;

pub mod users;

/// Mount all API routes
pub fn mount() -> Router {
    Router::new().merge(users::mount())
}
