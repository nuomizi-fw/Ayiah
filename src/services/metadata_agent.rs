use crate::{
    entities::{CreateVideoMetadata, MediaItem, MediaType, VideoMetadata},
    scraper::{MediaDetails, ScraperManager},
};
use std::sync::Arc;
use tracing::{debug, error, info, warn};

/// Metadata agent service for fetching and saving metadata
pub struct MetadataAgent {
    scraper_manager: Arc<ScraperManager>,
    db: sqlx::SqlitePool,
}

impl MetadataAgent {
    /// Create a new metadata agent
    pub fn new(scraper_manager: Arc<ScraperManager>, db: sqlx::SqlitePool) -> Self {
        Self {
            scraper_manager,
            db,
        }
    }

    /// Fetch and save metadata for a media item
    pub async fn fetch_and_save_metadata(
        &self,
        media_item: &MediaItem,
    ) -> Result<VideoMetadata, MetadataAgentError> {
        info!(
            "Fetching metadata for {} (ID: {})",
            media_item.title, media_item.id
        );

        // Extract year from title if present (e.g., "Movie Title (2023)")
        let (title, year) = self.parse_title_and_year(&media_item.title);

        // Search for the media
        let search_results = self
            .scraper_manager
            .search(&title, year)
            .await
            .map_err(|e| {
                error!("Failed to search for {}: {}", title, e);
                MetadataAgentError::SearchFailed(e.to_string())
            })?;

        // Filter results by media type
        let matching_result = search_results
            .into_iter()
            .find(|result| {
                matches!(
                    (media_item.media_type, result.media_type()),
                    (MediaType::Movie, crate::scraper::MediaType::Movie)
                        | (MediaType::Tv, crate::scraper::MediaType::Tv)
                )
            })
            .ok_or_else(|| {
                warn!("No matching results found for {}", title);
                MetadataAgentError::NoMatchingResults
            })?;

        debug!(
            "Found matching result: {} (Provider: {})",
            matching_result.title(),
            matching_result.provider()
        );

        // Get detailed metadata
        let details = self
            .scraper_manager
            .get_details(&matching_result)
            .await
            .map_err(|e| {
                error!("Failed to get details: {}", e);
                MetadataAgentError::DetailsFailed(e.to_string())
            })?;

        // Convert to database format and save
        let metadata = self.save_metadata(media_item.id, details).await?;

        info!(
            "Successfully saved metadata for {} (ID: {})",
            media_item.title, media_item.id
        );

        Ok(metadata)
    }

    /// Save metadata to database
    async fn save_metadata(
        &self,
        media_item_id: i64,
        details: MediaDetails,
    ) -> Result<VideoMetadata, MetadataAgentError> {
        let create_metadata = match details {
            MediaDetails::Movie(movie) => CreateVideoMetadata {
                media_item_id,
                tmdb_id: movie
                    .external_ids
                    .tmdb_id
                    .and_then(|id| id.parse().ok()),
                tvdb_id: movie
                    .external_ids
                    .tvdb_id
                    .and_then(|id| id.parse().ok()),
                imdb_id: movie.external_ids.imdb_id,
                overview: movie.overview,
                poster_path: movie.poster_path,
                backdrop_path: movie.backdrop_path,
                release_date: movie.release_date,
                runtime: movie.runtime,
                vote_average: movie.vote_average,
                vote_count: movie.vote_count,
                genres: movie.genres,
            },
            MediaDetails::Tv(tv) => CreateVideoMetadata {
                media_item_id,
                tmdb_id: tv.external_ids.tmdb_id.and_then(|id| id.parse().ok()),
                tvdb_id: tv.external_ids.tvdb_id.and_then(|id| id.parse().ok()),
                imdb_id: tv.external_ids.imdb_id,
                overview: tv.overview,
                poster_path: tv.poster_path,
                backdrop_path: tv.backdrop_path,
                release_date: tv.first_air_date,
                runtime: tv.episode_run_time.first().copied(),
                vote_average: tv.vote_average,
                vote_count: tv.vote_count,
                genres: tv.genres,
            },
            MediaDetails::Anime(_) => {
                return Err(MetadataAgentError::UnsupportedMediaType(
                    "Anime not yet supported".to_string(),
                ))
            }
        };

        VideoMetadata::upsert(&self.db, create_metadata)
            .await
            .map_err(|e| {
                error!("Failed to save metadata to database: {}", e);
                MetadataAgentError::DatabaseError(e.to_string())
            })
    }

    /// Parse title and year from a string like "Movie Title (2023)"
    fn parse_title_and_year(&self, title: &str) -> (String, Option<i32>) {
        let re = regex::Regex::new(r"^(.+?)\s*\((\d{4})\)\s*$").expect("Invalid regex");

        if let Some(captures) = re.captures(title) {
            let title = captures.get(1).map(|m| m.as_str().to_string()).unwrap_or_else(|| title.to_string());
            let year = captures
                .get(2)
                .and_then(|m| m.as_str().parse().ok());
            (title, year)
        } else {
            (title.to_string(), None)
        }
    }

    /// Refresh metadata for an existing media item
    pub async fn refresh_metadata(
        &self,
        media_item_id: i64,
    ) -> Result<VideoMetadata, MetadataAgentError> {
        let media_item = MediaItem::find_by_id(&self.db, media_item_id)
            .await
            .map_err(|e| MetadataAgentError::DatabaseError(e.to_string()))?
            .ok_or(MetadataAgentError::MediaItemNotFound)?;

        self.fetch_and_save_metadata(&media_item).await
    }

    /// Batch fetch metadata for multiple media items
    pub async fn batch_fetch_metadata(
        &self,
        media_items: Vec<MediaItem>,
    ) -> Vec<Result<VideoMetadata, MetadataAgentError>> {
        let mut results = Vec::new();

        for item in media_items {
            let result = self.fetch_and_save_metadata(&item).await;
            results.push(result);

            // Add a small delay to respect rate limits
            tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;
        }

        results
    }
}

/// Metadata agent errors
#[derive(Debug, thiserror::Error)]
pub enum MetadataAgentError {
    #[error("Search failed: {0}")]
    SearchFailed(String),

    #[error("No matching results found")]
    NoMatchingResults,

    #[error("Failed to get details: {0}")]
    DetailsFailed(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Media item not found")]
    MediaItemNotFound,

    #[error("Unsupported media type: {0}")]
    UnsupportedMediaType(String),
}
