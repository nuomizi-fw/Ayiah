use axum::Router;

pub fn mount() -> Router {
    Router::new().nest("/scrape", Router::new())
}
