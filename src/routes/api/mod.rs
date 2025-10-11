use axum::Router;

use crate::Ctx;

pub mod health;

/// Mount all API routes
pub fn mount() -> Router<Ctx> {
    Router::new().merge(health::mount())
}
