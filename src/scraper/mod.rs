pub mod provider;

use provider::MetadataProvider;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum OrganizeMethod {
    SoftLink,
    HardLink,
    Copy,
    Move,
}

impl Default for OrganizeMethod {
    fn default() -> Self {
        Self::SoftLink
    }
}

/// Media type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub enum MediaType {
    /// Video file
    Video,
    /// Book file
    Book,
    /// Music file
    Music,
    /// Comic file
    Comic,
}

/// Scraper provider enumeration
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub enum Provider {
    /// TMDB (The Movie Database)
    Tmdb,
    /// TVDB (The TV Database)
    Tvdb,
    /// Anilist
    Anilist,
    /// Bangumi
    Bangumi,
}

/// Main struct for Ayiah Scraper
pub struct AyiahScraper {
    pub organize_method: OrganizeMethod,
    pub source_path: PathBuf,
    pub target_path: PathBuf,
    pub metadata_provider: Box<dyn MetadataProvider>,
}

impl AyiahScraper {
    pub fn new(
        organize_method: OrganizeMethod,
        source_path: PathBuf,
        target_path: PathBuf,
        metadata_provider: Box<dyn MetadataProvider>,
    ) -> Self {
        Self {
            organize_method,
            source_path,
            target_path,
            metadata_provider,
        }
    }
}
