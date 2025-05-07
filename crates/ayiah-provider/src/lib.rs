use std::error::Error;

use serde::{Deserialize, Serialize};

pub mod anilist;
pub mod bangumi;
pub mod douban;
pub mod tmdb;
pub mod tvdb;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct VideoMetadata {}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BookMetadata {}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MusicMetadata {}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ComicMetadata {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
enum MediaMetadata {
    Video(VideoMetadata),
    Book(BookMetadata),
    Music(MusicMetadata),
    Comic(ComicMetadata),
}

pub trait MetadataProvider {
    fn fetch_metadata(
        &self,
    ) -> impl std::future::Future<Output = Result<MediaMetadata, Box<dyn Error>>> + Send;
}
