use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::provider::{MediaMetadata, MetadataProvider};

#[derive( Default,Debug, Clone, Serialize, Deserialize)]
pub struct BangumiProvider {
    pub api_key: String,
}

impl BangumiProvider {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[async_trait::async_trait]
impl MetadataProvider for BangumiProvider {
    async fn fetch_metadata(&self) -> Result<MediaMetadata, Box<dyn Error + Send + Sync>> {
        unimplemented!()
    }
}
