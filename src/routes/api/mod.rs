use axum::Router;

use crate::Ctx;

pub mod health;
pub mod provider;
pub mod scrape;

/// Mount all API routes
pub fn mount() -> Router<Ctx> {
    Router::new()
        .merge(health::mount())
        .merge(provider::mount())
        .merge(scrape::mount())
}
