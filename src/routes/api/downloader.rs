use axum::Router;

pub fn mount() -> Router {
    Router::new().nest("/downloader", Router::new())
}
