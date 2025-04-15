use axum::Router;

pub mod scrape;
pub mod users;

/// Mount all API routes
pub fn mount() -> Router {
    Router::new().merge(scrape::mount()).merge(users::mount())
}
