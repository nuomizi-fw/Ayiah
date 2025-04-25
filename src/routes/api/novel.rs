use axum::Router;

pub fn mount() -> Router {
    Router::new().nest("/novel", Router::new())
}
