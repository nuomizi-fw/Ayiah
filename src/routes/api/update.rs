use axum::Router;

pub fn mount() -> Router {
    Router::new().nest("/update", Router::new())
}
