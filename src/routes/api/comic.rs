use axum::Router;

pub fn mount() -> Router {
    Router::new().nest("/comic", Router::new())
}
