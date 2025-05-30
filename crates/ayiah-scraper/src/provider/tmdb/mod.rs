use std::error::Error;

use crate::provider::{MediaMetadata, MetadataProvider};

pub struct TmdbProvider {
    pub api_key: String,
}

impl TmdbProvider {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[async_trait::async_trait]
impl MetadataProvider for TmdbProvider {
    async fn fetch_metadata(&self) -> Result<MediaMetadata, Box<dyn Error + Send + Sync>> {
        unimplemented!()
    }
}
