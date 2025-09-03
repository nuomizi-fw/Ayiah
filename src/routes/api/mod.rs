use axum::Router;

use crate::Ctx;

pub mod provider;
pub mod scrape;

/// Mount all API routes
pub fn mount() -> Router<Ctx> {
    Router::new()
        .merge(provider::mount())
        .merge(scrape::mount())
}
