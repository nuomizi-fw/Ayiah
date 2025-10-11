pub mod anilist;
pub mod bangumi;
pub mod tmdb;
pub mod tvdb;

// Provider implementations will be exported in their respective modules
// pub use anilist::AniListProvider;
// pub use bangumi::BangumiProvider;
// pub use tmdb::TmdbProvider;
// pub use tvdb::TvdbProvider;

use crate::scraper::{RateLimiter, ScraperCache};
use reqwest::Client;
use std::sync::Arc;

/// Provider base configuration
#[derive(Debug, Clone)]
pub struct ProviderConfig {
    /// API key
    pub api_key: Option<String>,
    /// Base URL
    pub base_url: String,
    /// Rate limit configuration
    pub rate_limit: crate::scraper::RateLimitConfig,
    /// Cache TTL (seconds)
    pub cache_ttl: u64,
}

impl ProviderConfig {
    /// Create new configuration
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            api_key: None,
            base_url: base_url.into(),
            rate_limit: Default::default(),
            cache_ttl: 3600,
        }
    }

    /// Set API key
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Set rate limit
    pub fn with_rate_limit(mut self, rate_limit: crate::scraper::RateLimitConfig) -> Self {
        self.rate_limit = rate_limit;
        self
    }

    /// Set cache TTL
    pub fn with_cache_ttl(mut self, ttl_seconds: u64) -> Self {
        self.cache_ttl = ttl_seconds;
        self
    }
}

/// Provider base structure
pub struct ProviderBase {
    pub config: ProviderConfig,
    pub client: Client,
    pub rate_limiter: RateLimiter,
    pub cache: Arc<ScraperCache>,
}

impl ProviderBase {
    /// Create new provider base instance
    pub fn new(config: ProviderConfig, cache: Arc<ScraperCache>) -> Self {
        let rate_limiter = RateLimiter::new(config.rate_limit.clone());
        let client = Client::builder()
            .user_agent("Ayiah/0.1.0")
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            config,
            client,
            rate_limiter,
            cache,
        }
    }

    /// Execute rate-limited HTTP GET request
    pub async fn get_with_rate_limit(
        &self,
        provider_name: &str,
        url: &str,
    ) -> Result<reqwest::Response, crate::scraper::ScraperError> {
        let _guard = self
            .rate_limiter
            .acquire(provider_name)
            .await
            .map_err(|_e| {
                crate::scraper::ScraperError::RateLimit(std::time::Duration::from_secs(1))
            })?;

        self.client
            .get(url)
            .send()
            .await
            .map_err(crate::scraper::ScraperError::Network)
    }
}
