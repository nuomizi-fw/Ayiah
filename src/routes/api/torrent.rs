use axum::Router;

pub fn mount() -> Router {
    Router::new().nest("/torrent", Router::new())
}
