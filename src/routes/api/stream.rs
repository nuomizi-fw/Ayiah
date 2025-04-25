use axum::Router;

pub fn mount() -> Router {
    Router::new().nest("/stream", Router::new())
}
