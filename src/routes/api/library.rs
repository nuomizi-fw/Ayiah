use axum::Router;

pub fn mount() -> Router {
    Router::new().nest("/library", Router::new())
}
