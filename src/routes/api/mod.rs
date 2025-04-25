use axum::Router;

pub mod comic;
pub mod downloader;
pub mod history;
pub mod library;
pub mod music;
pub mod novel;
pub mod provider;
pub mod report;
pub mod scrape;
pub mod search;
pub mod stream;
pub mod torrent;
pub mod update;
pub mod users;
pub mod video;
pub mod watchlist;

/// Mount all API routes
pub fn mount() -> Router {
    Router::new()
        .merge(comic::mount())
        .merge(downloader::mount())
        .merge(history::mount())
        .merge(library::mount())
        .merge(music::mount())
        .merge(novel::mount())
        .merge(provider::mount())
        .merge(report::mount())
        .merge(search::mount())
        .merge(scrape::mount())
        .merge(stream::mount())
        .merge(torrent::mount())
        .merge(update::mount())
        .merge(users::mount())
        .merge(video::mount())
        .merge(watchlist::mount())
}
