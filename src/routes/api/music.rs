use axum::Router;

pub fn mount() -> Router {
    Router::new().nest("/music", Router::new())
}
