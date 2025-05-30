use std::error::Error;

use serde::{Deserialize, Serialize};

pub mod anilist;
pub mod bangumi;
pub mod tmdb;
pub mod tvdb;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoMetadata {
    pub title: Option<String>,
    pub original_title: Option<String>,
    pub description: Option<String>,
    pub overview: Option<String>,
    pub release_date: Option<String>,
    pub genres: Option<Vec<String>>,
    pub runtime: Option<u32>,           // in minutes
    pub rating: Option<f32>,            // e.g. 8.5/10
    pub content_rating: Option<String>, // e.g. PG-13, R, etc.
    pub poster_url: Option<String>,
    pub backdrop_url: Option<String>,
    pub director: Option<String>,
    pub writers: Option<Vec<String>>,
    pub cast: Option<Vec<String>>,
    pub production_companies: Option<Vec<String>>,
    pub countries: Option<Vec<String>>,
    pub languages: Option<Vec<String>>,
    pub imdb_id: Option<String>,
    pub tmdb_id: Option<String>,
    pub tvdb_id: Option<String>,
    pub anilist_id: Option<String>,
    pub douban_id: Option<String>,
    // Technical details
    pub resolution: Option<String>,
    pub bitrate: Option<u32>,
    pub codec: Option<String>,
    pub frame_rate: Option<f32>,
    pub audio_channels: Option<u8>,
    pub audio_codec: Option<String>,
    pub hdr: Option<bool>,
    pub file_size: Option<u64>,
    // TV specific fields
    pub season_number: Option<u32>,
    pub episode_number: Option<u32>,
    pub episode_title: Option<String>,
    pub total_seasons: Option<u32>,
    pub total_episodes: Option<u32>,
    pub network: Option<String>,
    pub air_date: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BookMetadata {
    pub title: Option<String>,
    pub original_title: Option<String>,
    pub subtitle: Option<String>,
    pub authors: Option<Vec<String>>,
    pub publisher: Option<String>,
    pub publication_date: Option<String>,
    pub description: Option<String>,
    pub isbn_10: Option<String>,
    pub isbn_13: Option<String>,
    pub page_count: Option<u32>,
    pub genres: Option<Vec<String>>,
    pub rating: Option<f32>,
    pub language: Option<String>,
    pub cover_url: Option<String>,
    pub series: Option<String>,
    pub series_position: Option<u32>,
    pub edition: Option<String>,
    pub format: Option<String>,      // hardcover, paperback, ebook, etc.
    pub file_size: Option<u64>,      // for ebooks
    pub file_format: Option<String>, // epub, mobi, pdf, etc.
    pub categories: Option<Vec<String>>,
    pub goodreads_id: Option<String>,
    pub douban_id: Option<String>,
    pub amazon_id: Option<String>,
    pub translator: Option<String>,
    pub country: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MusicMetadata {
    pub title: Option<String>,
    pub artists: Option<Vec<String>>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub composer: Option<String>,
    pub genre: Option<Vec<String>>,
    pub release_date: Option<String>,
    pub track_number: Option<u32>,
    pub total_tracks: Option<u32>,
    pub disc_number: Option<u32>,
    pub total_discs: Option<u32>,
    pub duration: Option<u32>,    // in seconds
    pub bit_depth: Option<u32>,   // e.g. 16bit, 24bit
    pub sample_rate: Option<u32>, // e.g. 44100, 48000, 96000
    pub bitrate: Option<u32>,     // in kbps
    pub format: Option<String>,   // mp3, flac, wav, etc.
    pub file_size: Option<u64>,
    pub album_cover_url: Option<String>,
    pub lyrics: Option<String>,
    pub isrc: Option<String>, // International Standard Recording Code
    pub catalog_number: Option<String>,
    pub label: Option<String>, // record label
    pub country: Option<String>,
    pub language: Option<String>,
    pub rating: Option<f32>,
    pub bpm: Option<u32>, // beats per minute
    pub musicbrainz_id: Option<String>,
    pub spotify_id: Option<String>,
    pub apple_music_id: Option<String>,
    pub compilation: Option<bool>,
    pub remastered: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComicMetadata {
    pub title: Option<String>,
    pub original_title: Option<String>,
    pub subtitle: Option<String>,
    pub series: Option<String>,
    pub volume: Option<u32>,
    pub issue_number: Option<u32>,
    pub alt_issue_number: Option<String>, // for special issues
    pub story_arc: Option<String>,
    pub description: Option<String>,
    pub publisher: Option<String>,
    pub imprint: Option<String>, // sub-brand of publisher
    pub publication_date: Option<String>,
    pub cover_date: Option<String>, // date shown on the cover
    pub writers: Option<Vec<String>>,
    pub artists: Option<Vec<String>>,
    pub pencillers: Option<Vec<String>>,
    pub inkers: Option<Vec<String>>,
    pub colorists: Option<Vec<String>>,
    pub letterers: Option<Vec<String>>,
    pub cover_artists: Option<Vec<String>>,
    pub editors: Option<Vec<String>>,
    pub page_count: Option<u32>,
    pub genres: Option<Vec<String>>,
    pub format: Option<String>, // single issue, trade paperback, etc.
    pub color: Option<bool>,    // color or black & white
    pub manga: Option<bool>,    // is it manga
    pub rating: Option<f32>,
    pub age_rating: Option<String>, // all ages, teen, mature, etc.
    pub cover_url: Option<String>,
    pub language: Option<String>,
    pub country: Option<String>,
    pub isbn: Option<String>, // for collected editions
    pub upc: Option<String>,  // barcode for single issues
    pub price: Option<f32>,
    pub comicvine_id: Option<String>,
    pub goodreads_id: Option<String>, // for collected editions
    pub file_size: Option<u64>,
    pub file_format: Option<String>, // cbz, cbr, pdf, etc.
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum MediaMetadata {
    Video(VideoMetadata),
    Book(BookMetadata),
    Music(MusicMetadata),
    Comic(ComicMetadata),
}

#[async_trait::async_trait]
pub trait MetadataProvider: Send + Sync {
    async fn fetch_metadata(&self) -> Result<MediaMetadata, Box<dyn Error + Send + Sync>>;
}
