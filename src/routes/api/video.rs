use axum::Router;

pub fn mount() -> Router {
    Router::new().nest("/video", Router::new())
}
