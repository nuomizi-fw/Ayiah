use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;

/// Rate limiter configuration
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    pub max_concurrent: usize,
    pub max_requests: usize,
    pub window_seconds: u64,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_concurrent: 5,
            max_requests: 40,
            window_seconds: 10,
        }
    }
}

#[derive(Debug, Clone)]
struct RequestRecord {
    timestamps: Vec<Instant>,
}

impl RequestRecord {
    const fn new() -> Self {
        Self {
            timestamps: Vec::new(),
        }
    }

    fn cleanup(&mut self, window: Duration) {
        let now = Instant::now();
        self.timestamps.retain(|&t| now.duration_since(t) < window);
    }

    const fn can_request(&self, max_requests: usize) -> bool {
        self.timestamps.len() < max_requests
    }

    fn record_request(&mut self) {
        self.timestamps.push(Instant::now());
    }

    fn next_available(&self, window: Duration, max_requests: usize) -> Option<Duration> {
        if self.timestamps.len() < max_requests {
            return None;
        }
        if let Some(&oldest) = self.timestamps.first() {
            let elapsed = Instant::now().duration_since(oldest);
            if elapsed < window {
                return Some(window - elapsed);
            }
        }
        None
    }
}

#[derive(Clone)]
pub struct RateLimiter {
    config: RateLimitConfig,
    semaphore: Arc<Semaphore>,
    records: Arc<DashMap<String, RequestRecord>>,
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new(RateLimitConfig::default())
    }
}

impl RateLimiter {
    #[must_use] 
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(config.max_concurrent)),
            config,
            records: Arc::new(DashMap::new()),
        }
    }

    pub async fn acquire(&self, provider: &str) -> Result<RateLimitGuard, String> {
        let permit = self
            .semaphore
            .clone()
            .acquire_owned()
            .await
            .map_err(|e| format!("Failed to acquire semaphore: {e}"))?;

        let window = Duration::from_secs(self.config.window_seconds);
        let key = provider.to_string();

        loop {
            let wait_duration = {
                let mut record = self
                    .records
                    .entry(key.clone())
                    .or_insert_with(RequestRecord::new);

                record.cleanup(window);

                if record.can_request(self.config.max_requests) {
                    record.record_request();
                    break;
                }
                record
                    .next_available(window, self.config.max_requests)
                    .unwrap_or(Duration::from_millis(100))
            };

            tracing::debug!(
                "Rate limit reached for provider '{}', waiting {:?}",
                provider,
                wait_duration
            );
            tokio::time::sleep(wait_duration).await;
        }

        Ok(RateLimitGuard { _permit: permit })
    }

    pub fn reset(&self, provider: &str) {
        self.records.remove(provider);
    }

    pub fn reset_all(&self) {
        self.records.clear();
    }

    #[must_use] 
    pub const fn config(&self) -> &RateLimitConfig {
        &self.config
    }
}

pub struct RateLimitGuard {
    _permit: tokio::sync::OwnedSemaphorePermit,
}
