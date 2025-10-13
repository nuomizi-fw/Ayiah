use moka::future::Cache;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Scraper cache key
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct CacheKey {
    pub provider: String,
    pub media_type: String,
    pub query: String,
}

impl CacheKey {
    pub fn new(
        provider: impl Into<String>,
        media_type: impl Into<String>,
        query: impl Into<String>,
    ) -> Self {
        Self {
            provider: provider.into(),
            media_type: media_type.into(),
            query: query.into(),
        }
    }
}

/// Scraper cache
#[derive(Clone)]
pub struct ScraperCache {
    cache: Cache<CacheKey, Vec<u8>>,
}

impl ScraperCache {
    /// Create a new cache instance (default TTL: 1 hour)
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(3600, 10000)
    }

    /// Create a cache instance with custom configuration
    #[must_use]
    pub fn with_config(ttl_seconds: u64, max_capacity: u64) -> Self {
        let cache = Cache::builder()
            .time_to_live(Duration::from_secs(ttl_seconds))
            .max_capacity(max_capacity)
            .build();

        Self { cache }
    }

    /// Store data to cache
    pub async fn set<T: Serialize + Send + Sync>(
        &self,
        key: CacheKey,
        value: &T,
    ) -> Result<(), String> {
        let serialized = serde_json::to_vec(value)
            .map_err(|e| format!("Failed to serialize cache entry: {e}"))?;

        self.cache.insert(key, serialized).await;
        Ok(())
    }

    /// Get data from cache
    pub async fn get<T: for<'de> Deserialize<'de>>(&self, key: &CacheKey) -> Option<T> {
        let data = self.cache.get(key).await?;
        serde_json::from_slice(&data).ok()
    }

    /// Invalidate a cache entry
    pub async fn invalidate(&self, key: &CacheKey) {
        self.cache.invalidate(key).await;
    }

    /// Clear all cache entries
    pub async fn clear(&self) {
        self.cache.invalidate_all();
        // Wait for all invalidation operations to complete
        self.cache.run_pending_tasks().await;
    }

    /// Get cache size (approximate)
    #[must_use]
    pub fn len(&self) -> u64 {
        self.cache.entry_count()
    }

    /// Check if cache is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.cache.entry_count() == 0
    }

    /// Run pending maintenance tasks
    pub async fn run_pending_tasks(&self) {
        self.cache.run_pending_tasks().await;
    }
}

impl Default for ScraperCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_set_and_get() {
        let cache = ScraperCache::new();
        let key = CacheKey::new("tmdb", "movie", "test");
        let value = vec!["movie1".to_string(), "movie2".to_string()];

        cache.set(key.clone(), &value).await.unwrap();
        let cached: Option<Vec<String>> = cache.get(&key).await;

        assert_eq!(cached, Some(value));
    }

    #[tokio::test]
    async fn test_cache_expiration() {
        let cache = ScraperCache::with_config(1, 1000); // 1 second TTL
        let key = CacheKey::new("tmdb", "movie", "test");
        let value = vec!["movie1".to_string()];

        cache.set(key.clone(), &value).await.unwrap();

        // Should succeed immediately
        assert!(cache.get::<Vec<String>>(&key).await.is_some());

        // Wait for expiration
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Should return None after expiration
        assert!(cache.get::<Vec<String>>(&key).await.is_none());
    }

    #[tokio::test]
    async fn test_cache_invalidate() {
        let cache = ScraperCache::new();
        let key = CacheKey::new("tmdb", "movie", "test");
        let value = vec!["movie1".to_string()];

        cache.set(key.clone(), &value).await.unwrap();
        assert!(cache.get::<Vec<String>>(&key).await.is_some());

        cache.invalidate(&key).await;
        assert!(cache.get::<Vec<String>>(&key).await.is_none());
    }

    #[tokio::test]
    async fn test_cache_clear() {
        let cache = ScraperCache::new();
        let key1 = CacheKey::new("tmdb", "movie", "test1");
        let key2 = CacheKey::new("tmdb", "movie", "test2");

        cache
            .set(key1.clone(), &vec!["movie1".to_string()])
            .await
            .unwrap();
        cache
            .set(key2.clone(), &vec!["movie2".to_string()])
            .await
            .unwrap();

        // Run pending tasks to ensure writes complete
        cache.run_pending_tasks().await;

        // Verify cache contains data
        assert!(cache.get::<Vec<String>>(&key1).await.is_some());
        assert!(cache.get::<Vec<String>>(&key2).await.is_some());

        // Clear cache
        cache.clear().await;

        // Verify cache is empty
        assert!(cache.get::<Vec<String>>(&key1).await.is_none());
        assert!(cache.get::<Vec<String>>(&key2).await.is_none());
        assert_eq!(cache.len(), 0);
    }
}
