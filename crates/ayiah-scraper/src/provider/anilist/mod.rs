use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::provider::{MediaMetadata, MetadataProvider};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AnilistProvider {
    pub api_key: String,
}

impl AnilistProvider {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[async_trait::async_trait]
impl MetadataProvider for AnilistProvider {
    async fn fetch_metadata(&self) -> Result<MediaMetadata, Box<dyn Error + Send + Sync>> {
        unimplemented!()
    }
}
