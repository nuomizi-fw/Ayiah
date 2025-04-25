use axum::Router;

pub fn mount() -> Router {
    Router::new().nest("/search", Router::new())
}
