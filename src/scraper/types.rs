use serde::{Deserialize, Serialize};

/// Media type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    Movie,
    Tv,
    Anime,
}

/// Generic media search result (includes all types)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "media_type", rename_all = "lowercase")]
pub enum MediaSearchResult {
    Movie(MovieSearchResult),
    Tv(TvSearchResult),
    Anime(AnimeSearchResult),
}

impl MediaSearchResult {
    /// Get ID
    pub fn id(&self) -> &str {
        match self {
            Self::Movie(m) => &m.id,
            Self::Tv(t) => &t.id,
            Self::Anime(a) => &a.id,
        }
    }

    /// Get title
    pub fn title(&self) -> &str {
        match self {
            Self::Movie(m) => &m.title,
            Self::Tv(t) => &t.name,
            Self::Anime(a) => &a.title,
        }
    }

    /// Get media type
    pub fn media_type(&self) -> MediaType {
        match self {
            Self::Movie(_) => MediaType::Movie,
            Self::Tv(_) => MediaType::Tv,
            Self::Anime(_) => MediaType::Anime,
        }
    }

    /// Get provider name
    pub fn provider(&self) -> &str {
        match self {
            Self::Movie(m) => &m.provider,
            Self::Tv(t) => &t.provider,
            Self::Anime(a) => &a.provider,
        }
    }
}

/// Generic media details (includes all types)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "media_type", rename_all = "lowercase")]
pub enum MediaDetails {
    Movie(MovieMetadata),
    Tv(TvMetadata),
    Anime(AnimeMetadata),
}

impl MediaDetails {
    /// Get ID
    pub fn id(&self) -> &str {
        match self {
            Self::Movie(m) => &m.id,
            Self::Tv(t) => &t.id,
            Self::Anime(a) => &a.id,
        }
    }

    /// Get title
    pub fn title(&self) -> &str {
        match self {
            Self::Movie(m) => &m.title,
            Self::Tv(t) => &t.name,
            Self::Anime(a) => &a.title,
        }
    }

    /// Get media type
    pub fn media_type(&self) -> MediaType {
        match self {
            Self::Movie(_) => MediaType::Movie,
            Self::Tv(_) => MediaType::Tv,
            Self::Anime(_) => MediaType::Anime,
        }
    }

    /// Get provider name
    pub fn provider(&self) -> &str {
        match self {
            Self::Movie(m) => &m.provider,
            Self::Tv(t) => &t.provider,
            Self::Anime(a) => &a.provider,
        }
    }
}

/// Movie search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovieSearchResult {
    /// Provider-specific ID
    pub id: String,
    /// Title
    pub title: String,
    /// Original title
    pub original_title: Option<String>,
    /// Release year
    pub year: Option<i32>,
    /// Poster path/URL
    pub poster_path: Option<String>,
    /// Overview
    pub overview: Option<String>,
    /// Vote average
    pub vote_average: Option<f64>,
    /// Provider name
    pub provider: String,
}

/// Movie metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovieMetadata {
    /// Provider-specific ID
    pub id: String,
    /// Title
    pub title: String,
    /// Original title
    pub original_title: Option<String>,
    /// Release date
    pub release_date: Option<String>,
    /// Runtime (minutes)
    pub runtime: Option<i32>,
    /// Overview
    pub overview: Option<String>,
    /// Poster path/URL
    pub poster_path: Option<String>,
    /// Backdrop path/URL
    pub backdrop_path: Option<String>,
    /// Vote average
    pub vote_average: Option<f64>,
    /// Vote count
    pub vote_count: Option<i32>,
    /// Genres
    pub genres: Vec<String>,
    /// Production companies
    pub production_companies: Vec<String>,
    /// Production countries
    pub production_countries: Vec<String>,
    /// Original language
    pub original_language: Option<String>,
    /// Provider name
    pub provider: String,
    /// External IDs
    pub external_ids: ExternalIds,
}

/// TV show search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TvSearchResult {
    /// Provider-specific ID
    pub id: String,
    /// Title
    pub name: String,
    /// Original title
    pub original_name: Option<String>,
    /// First air date
    pub first_air_date: Option<String>,
    /// Poster path/URL
    pub poster_path: Option<String>,
    /// Overview
    pub overview: Option<String>,
    /// Vote average
    pub vote_average: Option<f64>,
    /// Provider name
    pub provider: String,
}

/// TV show metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TvMetadata {
    /// Provider-specific ID
    pub id: String,
    /// Title
    pub name: String,
    /// Original title
    pub original_name: Option<String>,
    /// First air date
    pub first_air_date: Option<String>,
    /// Last air date
    pub last_air_date: Option<String>,
    /// Overview
    pub overview: Option<String>,
    /// Poster path/URL
    pub poster_path: Option<String>,
    /// Backdrop path/URL
    pub backdrop_path: Option<String>,
    /// Vote average
    pub vote_average: Option<f64>,
    /// Vote count
    pub vote_count: Option<i32>,
    /// Genres
    pub genres: Vec<String>,
    /// Number of seasons
    pub number_of_seasons: Option<i32>,
    /// Number of episodes
    pub number_of_episodes: Option<i32>,
    /// Episode runtime (minutes)
    pub episode_run_time: Vec<i32>,
    /// Status
    pub status: Option<String>,
    /// Original language
    pub original_language: Option<String>,
    /// Production companies
    pub production_companies: Vec<String>,
    /// Provider name
    pub provider: String,
    /// External IDs
    pub external_ids: ExternalIds,
}

/// Episode metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeMetadata {
    /// Episode ID
    pub id: String,
    /// Episode name
    pub name: String,
    /// Season number
    pub season_number: i32,
    /// Episode number
    pub episode_number: i32,
    /// Air date
    pub air_date: Option<String>,
    /// Overview
    pub overview: Option<String>,
    /// Still path/URL
    pub still_path: Option<String>,
    /// Runtime (minutes)
    pub runtime: Option<i32>,
    /// Vote average
    pub vote_average: Option<f64>,
    /// Provider name
    pub provider: String,
}

/// Anime search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeSearchResult {
    /// Provider-specific ID
    pub id: String,
    /// Title
    pub title: String,
    /// English title
    pub title_english: Option<String>,
    /// Japanese title
    pub title_japanese: Option<String>,
    /// First air year
    pub year: Option<i32>,
    /// Poster path/URL
    pub poster_path: Option<String>,
    /// Overview
    pub overview: Option<String>,
    /// Score
    pub score: Option<f64>,
    /// Provider name
    pub provider: String,
}

/// Anime metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeMetadata {
    /// Provider-specific ID
    pub id: String,
    /// Title
    pub title: String,
    /// English title
    pub title_english: Option<String>,
    /// Japanese title
    pub title_japanese: Option<String>,
    /// Start date
    pub start_date: Option<String>,
    /// End date
    pub end_date: Option<String>,
    /// Overview
    pub overview: Option<String>,
    /// Poster path/URL
    pub poster_path: Option<String>,
    /// Backdrop path/URL
    pub backdrop_path: Option<String>,
    /// Score
    pub score: Option<f64>,
    /// Genres
    pub genres: Vec<String>,
    /// Episode count
    pub episodes: Option<i32>,
    /// Status
    pub status: Option<String>,
    /// Anime format (TV, Movie, OVA, etc.)
    pub format: Option<String>,
    /// Provider name
    pub provider: String,
    /// External IDs
    pub external_ids: ExternalIds,
}

/// External IDs
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExternalIds {
    /// IMDB ID
    pub imdb_id: Option<String>,
    /// TMDB ID
    pub tmdb_id: Option<String>,
    /// TVDB ID
    pub tvdb_id: Option<String>,
    /// AniList ID
    pub anilist_id: Option<String>,
    /// Bangumi ID
    pub bangumi_id: Option<String>,
    /// MyAnimeList ID
    pub mal_id: Option<String>,
}
