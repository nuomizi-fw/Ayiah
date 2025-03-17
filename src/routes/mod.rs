pub mod api;
pub mod openapi;

pub fn mount() -> axum::Router {
    axum::Router::new()
        .merge(openapi::mount())
        .nest("/api", api::mount())
}
