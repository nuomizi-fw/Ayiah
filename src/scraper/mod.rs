pub mod provider;

mod cache;
mod rate_limiter;
mod types;

pub use cache::ScraperCache;
pub use rate_limiter::{RateLimitConfig, RateLimiter};
pub use types::*;

use async_trait::async_trait;
use std::time::Duration;

/// Scraper result type
pub type Result<T> = std::result::Result<T, ScraperError>;

/// Scraper error types
#[derive(Debug, thiserror::Error)]
pub enum ScraperError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("API error: {status} - {message}")]
    Api { status: u16, message: String },

    #[error("Rate limit exceeded. Retry after: {0:?}")]
    RateLimit(Duration),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Cache error: {0}")]
    Cache(String),

    #[error("Configuration error: {0}")]
    Config(String),
}

/// Core trait for metadata providers
#[async_trait]
pub trait MetadataProvider: Send + Sync {
    /// Provider name
    fn name(&self) -> &str;

    /// Whether the provider requires an API key
    fn requires_api_key(&self) -> bool {
        false
    }

    /// Generic search
    ///
    /// Search for media based on query string and year, returning all matching results.
    /// Each provider decides which media types to search based on its capabilities.
    async fn search(&self, query: &str, year: Option<i32>) -> Result<Vec<MediaSearchResult>>;

    /// Get media details
    ///
    /// Retrieve complete metadata based on search results.
    async fn get_details(&self, result: &MediaSearchResult) -> Result<MediaDetails>;

    /// Get episode details
    ///
    /// Retrieve specific episode information for TV shows or anime.
    async fn get_episode_details(
        &self,
        series_id: &str,
        season: i32,
        episode: i32,
    ) -> Result<EpisodeMetadata>;
}

/// Scraper manager for managing multiple providers
pub struct ScraperManager {
    providers: Vec<Box<dyn MetadataProvider>>,
    cache: ScraperCache,
}

impl ScraperManager {
    /// Create a new scraper manager
    #[must_use] 
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
            cache: ScraperCache::new(),
        }
    }

    /// Add a provider
    pub fn add_provider(&mut self, provider: Box<dyn MetadataProvider>) {
        self.providers.push(provider);
    }

    /// Get all providers
    #[must_use] 
    pub fn providers(&self) -> &[Box<dyn MetadataProvider>] {
        &self.providers
    }

    /// Get cache
    #[must_use] 
    pub const fn cache(&self) -> &ScraperCache {
        &self.cache
    }

    /// Search media
    ///
    /// Query all registered providers and aggregate results.
    pub async fn search(&self, query: &str, year: Option<i32>) -> Result<Vec<MediaSearchResult>> {
        let mut all_results = Vec::new();

        for provider in &self.providers {
            match provider.search(query, year).await {
                Ok(results) => {
                    all_results.extend(results);
                }
                Err(e) => {
                    tracing::debug!("Provider {} search failed: {}", provider.name(), e);
                }
            }
        }

        if all_results.is_empty() {
            Err(ScraperError::NotFound(format!(
                "No provider could find: {query}"
            )))
        } else {
            Ok(all_results)
        }
    }

    /// Get media details
    ///
    /// Automatically select the correct provider based on search results.
    pub async fn get_details(&self, result: &MediaSearchResult) -> Result<MediaDetails> {
        let provider_name = result.provider();

        let provider = self
            .providers
            .iter()
            .find(|p| p.name() == provider_name)
            .ok_or_else(|| {
                ScraperError::Config(format!("Provider not found: {provider_name}"))
            })?;

        provider.get_details(result).await
    }

    /// Get episode details
    ///
    /// Retrieve specific episode information for TV shows or anime.
    pub async fn get_episode_details(
        &self,
        provider_name: &str,
        series_id: &str,
        season: i32,
        episode: i32,
    ) -> Result<EpisodeMetadata> {
        let provider = self
            .providers
            .iter()
            .find(|p| p.name() == provider_name)
            .ok_or_else(|| {
                ScraperError::Config(format!("Provider not found: {provider_name}"))
            })?;

        provider
            .get_episode_details(series_id, season, episode)
            .await
    }
}

impl Default for ScraperManager {
    fn default() -> Self {
        Self::new()
    }
}
