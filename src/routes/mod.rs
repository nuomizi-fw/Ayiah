use axum::Router;

use crate::Ctx;

pub mod api;

pub fn mount() -> Router<Ctx> {
    Router::new().nest("/api", api::mount())
}
