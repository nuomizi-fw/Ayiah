use axum::Router;

pub fn mount() -> Router {
    Router::new().nest("/watchlist", Router::new())
}
