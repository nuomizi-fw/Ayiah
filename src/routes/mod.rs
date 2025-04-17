use axum::Router;

pub mod api;
pub mod openapi;
pub mod service;

pub fn mount() -> Router {
    Router::new()
        .merge(openapi::mount())
        .nest("/api", api::mount())
}
